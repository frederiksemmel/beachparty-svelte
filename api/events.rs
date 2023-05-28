use beachparty::{get_all_events, Event};
use serde::Serialize;
use std::collections::HashMap;
use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

#[derive(Debug, Serialize)]
struct EventCategory {
    name: String,
    events: Vec<Event>,
}

fn events_api(events: Vec<Event>) -> Vec<EventCategory> {
    let mut events_per_type = HashMap::new();
    for event in events {
        let per_type: &mut Vec<Event> =
            events_per_type.entry(event.event_type.clone()).or_default();
        per_type.push(event);
        per_type.sort_by_key(|e| e.start)
    }
    events_per_type
        .into_iter()
        .map(|(k, v)| EventCategory { name: k, events: v })
        .collect()
}

pub async fn handler(_req: Request) -> Result<Response<Body>, Error> {
    let events = get_all_events().expect("recieved events");
    let api = events_api(events);
    let json = serde_json::to_string(&api)?;

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(json.into())?)
}
