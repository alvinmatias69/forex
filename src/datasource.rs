use std::collections::HashMap;

use serde::Deserialize;

use crate::conversion;

pub struct DataSource {}

#[derive(Deserialize)]
struct Response {
    rates: HashMap<String, f64>,
}

#[derive(Deserialize)]
struct ErrorReq {
    error: String,
}

impl conversion::DataSource for DataSource {
    fn fetch(&self, base: &str, symbols: &str) -> Result<HashMap<String, f64>, String> {
        let address: &str = &format!(
            "https://api.exchangeratesapi.io/latest?base={}&symbols={}",
            base, symbols
        )
        .to_owned();

        let result = match reqwest::blocking::get(address) {
            Ok(result) => result,
            Err(e) => return Err(e.to_string()),
        };

        if !result.status().is_success() {
            let error: ErrorReq = result.json().unwrap();
            return Err(error.error);
        }

        let resp: Response = match result.json() {
            Ok(result) => result,
            Err(_) => return Err(String::from("error parsing response")),
        };

        Ok(resp.rates)
    }
}
