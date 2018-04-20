extern crate chrono;
extern crate postgres;

use std::{thread, env};
use std::time::Duration;
use postgres::{Connection, TlsMode};
use chrono::Local;


fn main() {
    println!("Start daemon {}", Local::now());
    // for example,  "postgresql://anton:12345@localhost:5432/l09";
    let postgres_url = env::var("DBMETER_DB_DSN").unwrap();
    println!("postgres url: {}", postgres_url);
    start_monitoring(postgres_url);
}

fn start_monitoring(postgres_url: String) {
    let result = Connection::connect(postgres_url, TlsMode::None);
    match result {
        Ok(_result) => println!("ok"),
        Err(_error) => println!("error: {}", _error)
    }

    loop {
        thread::sleep(Duration::from_secs(1));
        println!("next ");
    }
}