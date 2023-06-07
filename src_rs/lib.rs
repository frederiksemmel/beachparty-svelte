pub mod auth;
pub mod config;
pub mod http_client;
pub mod sheets;

use std::collections::HashMap;

use config::Config;
use ical::parser::ical::component::IcalEvent;

// use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use google_sheets4::{hyper, hyper_rustls, Sheets};
use pomsky_macro::pomsky;
use regex::Regex;
use serde::Serialize;

fn _parse_event_property(event: &IcalEvent, name: &str) -> Option<String> {
    event
        .properties
        .iter()
        .filter(|p| p.name == name)
        .map(|p| p.value.to_owned())
        .next()
        .flatten()
}

fn _parse_names_from_description(desc: &str) -> Vec<String> {
    const CONFIRMED: &str = pomsky! {
        // variables
        let name = ([Letter] | " " | ['0' - '9'])*;
        let first = (name "\\, ")*;
        let last = name "\\n";
        "Confirmed Invitees: " :names(first last)
    };
    // println!("{}", CONFIRMED);
    let confirmed = Regex::new(CONFIRMED).unwrap();
    // desc.split("\n, ")
    let Some(names) = confirmed
        .captures(desc)
        .and_then(|c| c.name("names"))
         else {
        return vec![]
    };
    let names = names
        .as_str()
        .split("\\, ")
        .map(|n| n.replace("\\n", ""))
        .collect();
    // let matched = matched.replace("Confirmed Invitess: ", "");
    // println!("{}", invitees);
    names
}

fn _parse_type_from_summary(summ: &str) -> Option<String> {
    const TYPE: &str = pomsky! {
        // variables
        let number = (['0' - '9'])+;
        let name = ([Letter] | " " | number)*;
        :eventtype(name) " (" number " of " number " spots filled)"
    };
    // println!("{}", CONFIRMED);
    let event_type_re = Regex::new(TYPE).unwrap();
    // desc.split("\n, ")
    event_type_re
        .captures(summ)
        .and_then(|c| c.name("eventtype"))
        .map(|c| c.as_str().to_owned())
}

#[derive(Debug, Serialize)]
pub struct Registration {
    pub activity: String,
    pub name: String,
    pub start: String,
}

fn get_col(row: &[serde_json::Value], i: usize) -> anyhow::Result<String> {
    let value = row.get(i).ok_or(anyhow::format_err!("no value at {i}"))?;
    let serde_json::Value::String(s) = value else {
        return Err(anyhow::format_err!("{value} not a string"))
    };
    Ok(s.to_owned())
}

impl Registration {
    pub(crate) fn try_parse_from_row(row: &[serde_json::Value]) -> anyhow::Result<Self> {
        let activity = get_col(row, 0)?;
        let name = get_col(row, 1)?;
        let start = get_col(row, 2)?;
        // dbg!(&start);
        // let start = NaiveTime::parse_from_str(&start, "%H:%M")?;
        // let date = NaiveDate::from_ymd_opt(2023, 9, 16).unwrap();
        // let start = NaiveDateTime::new(date, start);
        Ok(Registration {
            activity,
            name,
            start,
        })
    }

    pub fn try_from_form(mut form: HashMap<String, String>) -> anyhow::Result<Self> {
        let activity = form
            .remove("activity")
            .ok_or(anyhow::format_err!("No field activity"))?;
        let name = form
            .remove("name")
            .ok_or(anyhow::format_err!("No field name"))?;
        let start = form
            .remove("start")
            .ok_or(anyhow::format_err!("No field start"))?;
        // let start = NaiveTime::parse_from_str(&start, "%H:%M")?;
        // let date = NaiveDate::from_ymd_opt(2023, 9, 16).unwrap();
        // let start = NaiveDateTime::new(date, start);
        Ok(Registration {
            activity,
            name,
            start,
        })
    }
}

pub async fn get_registrations_from_sheets(
    hub: &Sheets<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>>,
    config: &Config,
) -> Result<Vec<Registration>, anyhow::Error> {
    let events_sheet = sheets::read_sheet(hub, config, "Registrations!A:C").await?;

    let mut events = vec![];
    for row in events_sheet.into_iter().skip(1) {
        let event = Registration::try_parse_from_row(&row)?;
        events.push(event);
    }
    Ok(events)
}

#[derive(Debug, Serialize)]
pub struct EventInfo {
    name: String,
    description: String,
    location: String,
    duration: String,
    max_people_per_slot: usize,
    slots: Vec<String>,
}

impl EventInfo {
    pub fn try_parse_from_row(row: &[serde_json::Value]) -> anyhow::Result<Self> {
        let name = get_col(row, 0)?;
        let description = get_col(row, 1)?;
        let location = get_col(row, 2)?;
        let duration = get_col(row, 3)?;
        let max_people_per_slot = get_col(row, 4)?.parse()?;
        let slots = get_col(row, 5)?;
        let slots = slots.split(", ").map(|s| s.to_owned()).collect();
        Ok(EventInfo {
            name,
            description,
            location,
            duration,
            max_people_per_slot,
            slots,
        })
    }
}

pub async fn get_event_info(
    hub: &Sheets<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>>,
    config: &Config,
) -> anyhow::Result<Vec<EventInfo>> {
    let info_sheet = sheets::read_sheet(hub, config, "Info!A:F").await?;

    let mut infos = vec![];
    for row in info_sheet.into_iter().skip(1) {
        let info = EventInfo::try_parse_from_row(&row)?;
        infos.push(info);
    }
    Ok(infos)
}

pub async fn add_registration_to_sheet(
    hub: &Sheets<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>>,
    config: &Config,
    registration: Registration,
) -> anyhow::Result<()> {
    let values = vec![registration.activity, registration.name, registration.start];
    sheets::write_to_sheet(hub, config, "Registrations!A:C", values).await?;
    Ok(())
}
