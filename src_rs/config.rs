use std::env;

pub struct Config {
    pub service_account_key: String,
    pub sheet_id: String,
}

impl Default for Config {
    fn default() -> Config {
        let service_account_key = env::var("SERVICE_ACCOUNT_KEY").unwrap();
        Config {
            service_account_key,
            sheet_id: String::from("1hoIEXRMEUXHdHIiCUcoCGqhBwuqVocHX4Q3wHkd0pBo"),
        }
    }
}
