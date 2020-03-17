use clap::ArgMatches;

pub trait Conversion {
    fn set_cache(&mut self, skip_cache: bool);
    fn set_precision(&mut self, precision: usize);
    fn convert(&self, amount: f64, base: &str, target: Vec<&str>);
}

pub struct Handler<T: Conversion> {
    pub conversion: T,
}

impl<T> Handler<T>
where
    T: Conversion,
{
    pub fn run(&mut self, args: ArgMatches) {
        let precision = args.value_of("precision").unwrap();
        let precision = match precision.parse::<usize>() {
            Ok(precision) => precision,
            Err(_) => return self.show_error(precision),
        };
        self.conversion.set_precision(precision);

        let skip_cache = args.is_present("cache");
        self.conversion.set_cache(skip_cache);

        let amount = args.value_of("amount").unwrap();
        let amount = match amount.parse::<f64>() {
            Ok(result) => result,
            Err(_) => return self.show_error(amount),
        };

        let base = args.value_of("base").unwrap();
        let target: Vec<&str> = args.values_of("target").unwrap().collect();

        self.conversion.convert(amount, base, target);
    }

    fn show_error(&self, arg: &str) {
        println!("error: {} is not a valid number", arg);
    }
}
