use std::collections::HashMap;
use std::time::{Duration, SystemTime};

use crate::handler;

pub trait DataSource {
    fn fetch(&self, base: &str, symbols: &str) -> Result<HashMap<String, f64>, String>;
}

#[derive(Debug)]
pub struct RateCache {
    pub rate: f64,
    pub modified: SystemTime,
}

pub trait Cache {
    fn get(&self, base: &str, target: &str) -> Result<RateCache, String>;
    fn set(&self, base: &str, target: HashMap<String, f64>) -> Result<(), String>;
}

pub struct Conversion<T: DataSource, C: Cache> {
    pub datasource: T,
    pub cache: C,
    pub skip_cache: bool,
    pub precision: usize,
}

struct CacheResult {
    target: Vec<String>,
    result: HashMap<String, f64>,
}

impl<T, C> handler::Conversion for Conversion<T, C>
where
    T: DataSource,
    C: Cache,
{
    fn set_cache(&mut self, skip_cache: bool) {
        self.skip_cache = skip_cache;
    }

    fn set_precision(&mut self, precision: usize) {
        self.precision = precision;
    }

    fn convert(&self, amount: f64, base: &str, target: Vec<&str>) {
        let base = &base.to_ascii_uppercase();
        let mut fetch_target: Vec<String> = target.iter().map(|item| item.to_string()).collect();
        let mut result: HashMap<String, f64> = HashMap::new();

        if !self.skip_cache {
            let cache = self.get_partial_cache(amount, base, fetch_target);
            fetch_target = cache.target;
            result = cache.result;
        }

        if fetch_target.len() >= 1 {
            let symbols = fetch_target.join(",");
            let data = match self.datasource.fetch(base, &symbols) {
                Ok(result) => result,
                Err(e) => {
                    println!("error: {}", e);
                    return;
                }
            };

            for (k, v) in data.iter() {
                result.insert(k.to_string(), amount * v);
            }

            if let Err(error) = self.cache.set(base, data) {
                println!("error: {}", error);
            };
        }

        self.print_result(amount, base, result);
    }
}

impl<T, C> Conversion<T, C>
where
    T: DataSource,
    C: Cache,
{
    fn print_result(&self, amount: f64, base: &str, result: HashMap<String, f64>) {
        for (key, value) in result.iter() {
            println!("‣ {0:.1$} {2}", value, self.precision, key);
        }
        println!("converting {0:.1$} {2}", amount, self.precision, base);
    }

    fn get_partial_cache(&self, amount: f64, base: &str, target: Vec<String>) -> CacheResult {
        let current_time = SystemTime::now();
        let one_day_duration = Duration::from_secs(3600 * 24);

        let mut fetch_target: Vec<String> = Vec::new();
        let mut result: HashMap<String, f64> = HashMap::new();

        for item in target.iter() {
            let current_target = item.to_ascii_uppercase();
            let cache = match self.cache.get(base, &current_target) {
                Ok(result) => result,
                Err(_) => {
                    fetch_target.push(current_target);
                    continue;
                }
            };

            let duration = match current_time.duration_since(cache.modified) {
                Ok(result) => result,
                Err(_) => {
                    fetch_target.push(current_target);
                    continue;
                }
            };

            if duration > one_day_duration {
                fetch_target.push(current_target);
                continue;
            }

            result.insert(current_target, amount * cache.rate);
        }

        CacheResult {
            target: fetch_target,
            result,
        }
    }
}
