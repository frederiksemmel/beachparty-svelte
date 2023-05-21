use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};

use beachparty::get_all_events;

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(_req: Request) -> Result<Response<Body>, Error> {
    let events = get_all_events().expect("recieved events");
    let json = serde_json::to_string(&events)?;

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(json.into())?)
}
