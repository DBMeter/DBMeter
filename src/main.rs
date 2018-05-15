extern crate chrono;
extern crate postgres;
#[macro_use]
extern crate lazy_static;

mod config;
mod postgres_monitoring;

use chrono::Local;
use config::Config;
use postgres_monitoring::start_monitoring;

fn main() {
    println!("Start daemon {}", Local::now());

    let config = Config::new();

    if let &Some(ref dsn) = config.get_postgres() {
        start_monitoring(dsn.to_string());
    }
}
