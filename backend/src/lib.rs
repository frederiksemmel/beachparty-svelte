use ical::parser::ical::component::IcalEvent;
use std::io::BufReader;

use chrono::prelude::*;
use chrono::DateTime;
use pomsky_macro::pomsky;
use regex::Regex;
use serde::Serialize;

fn parse_event_property(event: &IcalEvent, name: &str) -> Option<String> {
    event
        .properties
        .iter()
        .filter(|p| p.name == name)
        .map(|p| p.value.to_owned())
        .next()
        .flatten()
}

fn parse_names_from_description(desc: &str) -> Vec<String> {
    const CONFIRMED: &str = pomsky! {
        // variables
        let name = ([Letter] | " ")*;
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

#[derive(Debug, Serialize)]
pub struct Event {
    pub title: String,
    pub attendees: Vec<String>,
    pub location: String,
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}

impl Event {
    fn try_parse_from_ical(event: &IcalEvent) -> Option<Self> {
        let title = parse_event_property(event, "SUMMARY").unwrap();
        if title.starts_with("Canceled") {
            return None;
        }
        let description = parse_event_property(event, "DESCRIPTION").unwrap();
        let location = parse_event_property(event, "LOCATION").unwrap();
        let start = parse_event_property(event, "DTSTART").unwrap();
        let start = Utc.datetime_from_str(&start, "%Y%m%dT%H%M%SZ").unwrap();
        let end = parse_event_property(event, "DTEND").unwrap();
        let end = Utc.datetime_from_str(&end, "%Y%m%dT%H%M%SZ").unwrap();
        let attendees = parse_names_from_description(&description);
        Some(Event {
            title,
            attendees,
            location,
            start,
            end,
        })
    }
}

pub fn get_all_events() -> anyhow::Result<Vec<Event>> {
    let body = BufReader::new(ureq::get(
        "https://calendar.google.com/calendar/ical/semmelbeachparty%40gmail.com/public/basic.ics",
    )
    .call()?
    .into_reader());
    // println!("{:?}", body);
    // let buf = BufReader::new(File::open("/home/frederik/party llavaneras/basic.ics").unwrap());

    let reader = ical::IcalParser::new(body);

    let mut events = vec![];

    for cal in reader {
        let cal = cal.unwrap();
        for event in cal.events {
            if let Some(event) = Event::try_parse_from_ical(&event) {
                println!("{:#?}", event);
                events.push(event)
            }
        }
    }
    Ok(events)
}
