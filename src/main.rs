extern crate chrono;
extern crate postgres;

mod config;
mod postgres_monitoring;

use chrono::Local;
use config::Config;
use postgres_monitoring::start_monitoring;

fn main() {
    println!("Start daemon {}", Local::now());

    let config = Config::new();

    if let Some(dsn) = config.get_postgres() {
        start_monitoring(dsn);
    }
}
