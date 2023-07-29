use beachparty::{
    auth, config, get_event_info, get_registrations_from_sheets, http_client, EventInfo,
    Registration,
};
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
// use chrono::NaiveDateTime;
use google_sheets4::Sheets;
use serde::Serialize;
use std::collections::HashMap;
use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

#[derive(Debug, Serialize)]
struct Event {
    start: String,
    attendees: Vec<String>,
}

#[derive(Debug, Serialize)]
struct EventCategory {
    name: String,
    events: Vec<Event>,
}

#[derive(Debug, Serialize)]
struct Api {
    info: Vec<EventInfo>,
    events: Vec<EventCategory>,
}

fn events_api(registrations: Vec<Registration>) -> Vec<EventCategory> {
    let mut map: HashMap<String, HashMap<String, Vec<String>>> = HashMap::new();
    for reg in registrations {
        let per_type: &mut HashMap<String, Vec<String>> = map.entry(reg.activity).or_default();
        let attendees = per_type.entry(reg.start).or_default();
        attendees.push(reg.name);
    }
    let mut event_cats: Vec<_> = map
        .into_iter()
        .map(|(name, events)| {
            let mut events: Vec<Event> = events
                .into_iter()
                .map(|(start, attendees)| Event { start, attendees })
                .collect();
            events.sort_by_key(|e| e.start.clone());
            EventCategory { name, events }
        })
        .collect();
    event_cats.sort_by_key(|cat| {
        let date = if cat.name.to_lowercase().contains("thursday") {
            NaiveDate::from_ymd_opt(2023, 9, 14).unwrap()
        } else if cat.name.to_lowercase().contains("friday") {
            NaiveDate::from_ymd_opt(2023, 9, 15).unwrap()
        } else {
            NaiveDate::from_ymd_opt(2023, 9, 16).unwrap()
        };
        let time = cat
            .events
            .iter()
            .map(|e| {
                NaiveTime::parse_from_str(&e.start, "%H:%M")
                    .unwrap_or(NaiveTime::from_hms_opt(9, 0, 0).unwrap())
            })
            .min()
            .unwrap_or(NaiveTime::from_hms_opt(9, 0, 0).unwrap());
        NaiveDateTime::new(date, time)
    });
    event_cats
}

pub async fn handler(_req: Request) -> Result<Response<Body>, Error> {
    // dbg!(&req);
    let config = config::Config::default();
    let client = http_client::http_client();
    let auth = auth::auth(&config, client.clone()).await;
    let hub = Sheets::new(client.clone(), auth);
    let registrations = get_registrations_from_sheets(&hub, &config)
        .await
        .expect("recieved events");
    let info = get_event_info(&hub, &config)
        .await
        .expect("recieved event info");
    let events = events_api(registrations);
    let api = Api { info, events };
    let json = serde_json::to_string(&api)?;

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(json.into())?)
}
