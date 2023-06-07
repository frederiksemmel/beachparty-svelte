use beachparty::{
    auth, config, get_event_info, get_events_from_sheets, http_client, EventInfo, Registration,
};
use google_sheets4::Sheets;
use serde::Serialize;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let config = config::Config::default();
    let client = http_client::http_client();
    let auth = auth::auth(&config, client.clone()).await;
    let hub = Sheets::new(client.clone(), auth);
    let events = get_events_from_sheets(&hub, &config)
        .await
        .expect("recieved events");
    let info = get_event_info(&hub, &config)
        .await
        .expect("recieved event info");
    let events = events_api(events);
    let api = Api { info, events };
    let json = serde_json::to_string(&api)?;
    println!("{json:?}");
    Ok(())
}

#[derive(Debug, Serialize)]
struct EventCategory {
    name: String,
    events: Vec<Registration>,
}

#[derive(Debug, Serialize)]
struct Api {
    info: Vec<EventInfo>,
    events: Vec<EventCategory>,
}

fn events_api(events: Vec<Registration>) -> Vec<EventCategory> {
    let mut events_per_type = HashMap::new();
    for event in events {
        let per_type: &mut Vec<Registration> =
            events_per_type.entry(event.event.clone()).or_default();
        per_type.push(event);
        per_type.sort_by_key(|e| e.start.clone())
    }
    events_per_type
        .into_iter()
        .map(|(k, v)| EventCategory { name: k, events: v })
        .collect()
}
