use fs::File;
use std::collections::HashMap;
use std::fs;
use std::io::prelude::*;

use crate::conversion;

pub struct Cache {}

const BASEPATH: &str = "/tmp/forex";

impl conversion::Cache for Cache {
    fn get(&self, base: &str, target: &str) -> Result<conversion::RateCache, String> {
        let path = format!("{}/{}/{}", BASEPATH, base, target);
        let mut file = match File::open(path) {
            Ok(file) => file,
            Err(_) => return Err(String::from("cache not available")),
        };

        let metadata = match file.metadata() {
            Ok(metadata) => metadata,
            Err(_) => return Err(String::from("unable to get cache metadata")),
        };

        let modified = match metadata.modified() {
            Ok(modified) => modified,
            Err(_) => return Err(String::from("unable to get cache modifier data")),
        };

        let mut content = String::new();
        if let Err(_) = file.read_to_string(&mut content) {
            return Err(String::from("unable to read cache"));
        }

        let rate = match content.parse::<f64>() {
            Ok(rate) => rate,
            Err(_) => return Err(String::from("unable to parse cache content")),
        };

        Ok(conversion::RateCache { rate, modified })
    }

    fn set(&self, base: &str, target: HashMap<String, f64>) -> Result<(), String> {
        let path = format!("{}/{}", BASEPATH, base);
        if let Err(_) = fs::create_dir_all(path) {
            return Err(String::from("error creating cache path"));
        }

        for (key, value) in target.iter() {
            let path = format!("{}/{}/{}", BASEPATH, base, key);
            if let Err(_) = fs::write(path, value.to_string()) {
                return Err(String::from("error writing cache"));
            }
        }

        Ok(())
    }
}
