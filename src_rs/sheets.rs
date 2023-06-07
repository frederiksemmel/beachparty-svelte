use crate::config::Config;
use google_sheets4::{api::ValueRange, hyper, hyper_rustls, Sheets};

pub async fn read_sheet(
    hub: &Sheets<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>>,
    config: &Config,
    range: &str,
) -> anyhow::Result<Vec<Vec<serde_json::Value>>> {
    let (_, value_range) = hub
        .spreadsheets()
        .values_get(&config.sheet_id, range)
        .doit()
        .await?;

    let Some(values) = value_range.values else {
        return Err(anyhow::format_err!("Sheet {range} is empty"))
    };
    Ok(values)
}

pub async fn write_to_sheet(
    hub: &Sheets<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>>,
    config: &Config,
    range: &str,
    values: Vec<String>,
) -> anyhow::Result<()> {
    let values = values.into_iter().map(serde_json::Value::String).collect();
    let values = Some(vec![values]);
    let request = ValueRange {
        major_dimension: None,
        range: None,
        values,
    };
    let (res, value_range) = hub
        .spreadsheets()
        .values_append(request, &config.sheet_id, range)
        .value_input_option("RAW")
        .doit()
        .await?;
    dbg!(res);
    dbg!(value_range);
    Ok(())
}
