mod cache;
mod conversion;
mod datasource;
mod handler;

#[macro_use]
extern crate clap;
use clap::App;

const BASE_PRECISION: usize = 2;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let args = App::from_yaml(yaml).get_matches();

    let ds = datasource::DataSource {};
    let ca = cache::Cache {};
    let conv = conversion::Conversion {
        datasource: ds,
        cache: ca,
        skip_cache: false,
        precision: BASE_PRECISION,
    };
    let mut handle = handler::Handler { conversion: conv };
    handle.run(args);
}
