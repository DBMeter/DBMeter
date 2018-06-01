extern crate chrono;
extern crate postgres;

extern crate elastic;

#[cfg(test)]
#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate elastic_derive;

#[macro_use]
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

mod config;
mod db_drivers;
mod export;
mod monitoring;

use chrono::Local;
use config::Config;
use monitoring::start_monitoring;

fn main() {
    println!("Start daemon {}", Local::now());

    let config = Config::new();

    if let &Some(ref dsn) = config.get_postgres() {
        start_monitoring(dsn.to_string());
    }
}
