use postgres::{Connection, TlsMode};
use std::thread;
use std::time::Duration;

pub fn start_monitoring(postgres_url: String) {
    println!("postgres url: {}", postgres_url);

    let result = Connection::connect(postgres_url, TlsMode::None);
    match result {
        Ok(_result) => println!("Connected"),
        Err(_error) => println!("Connection error: {}", _error),
    }

    loop {
        thread::sleep(Duration::from_secs(1));
        println!("next ");
    }
}
