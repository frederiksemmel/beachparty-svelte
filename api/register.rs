use std::collections::HashMap;

use beachparty::{add_registration_to_sheet, auth, config, http_client, Registration};
use google_sheets4::Sheets;
use hyper::header;
use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

async fn register(req: Request) -> anyhow::Result<()> {
    let config = config::Config::default();
    let client = http_client::http_client();
    let auth = auth::auth(&config, client.clone()).await;
    let hub = Sheets::new(client.clone(), auth);
    let body_bytes = req.into_body();
    let form_data: HashMap<String, String> = serde_urlencoded::from_bytes(&body_bytes)?;
    dbg!(&form_data);
    let registration = Registration::try_from_form(form_data);
    dbg!(&registration);
    add_registration_to_sheet(&hub, &config, registration?).await
}

pub async fn handler(req: Request) -> Result<Response<Body>, Error> {
    let host = req
        .headers()
        .get(header::HOST)
        .ok_or(anyhow::format_err!("error"))?;
    let redirect_url = format!("http://{}#partytraining", host.to_str()?);
    let redirect_url_error = format!("http://{}#partytraining?error", host.to_str()?);
    match register(req).await {
        Ok(()) => Ok(Response::builder()
            .status(StatusCode::SEE_OTHER)
            .header(header::LOCATION, redirect_url)
            .body(Body::Empty)?),
        Err(e) => {
            println!("Error {e}");
            Ok(Response::builder()
                .status(StatusCode::SEE_OTHER)
                .header(header::LOCATION, redirect_url_error)
                .body(Body::Empty)?)
        }
    }
    // Ok(Response::builder()
    //     .status(StatusCode::NO_CONTENT)
    //     .body(Body::Empty)?)
}
