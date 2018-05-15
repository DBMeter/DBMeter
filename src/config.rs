/// Config reads keys from environment variables or .env file
extern crate dotenv;

use self::dotenv::dotenv;
use std::env;

pub struct Config {
    postgres_dsn: Option<String>,
}

impl Config {
    pub fn new() -> Config {
        // reading from dontenv file to environment variables
        dotenv().ok();

        let mut config = Config { postgres_dsn: None };

        if let Ok(_postgres_dsn) = env::var("POSTGRES_DSN") {
            config.postgres_dsn = Some(_postgres_dsn)
        }

        config
    }

    pub fn get_postgres<'a>(&'a self) -> &'a Option<String> {
        &self.postgres_dsn
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::sync::Mutex;

    lazy_static! {
        pub static ref ENV_MUTEX: Mutex<()> = Mutex::new(());
    }

    #[test]
    fn get_postgres() {
        let _mutex_guard = ENV_MUTEX.lock().unwrap();

        env::set_var(
            "POSTGRES_DSN",
            "postgresql://anton:12345@localhost:5432/l09",
        );
        match Config::new().get_postgres() {
            &Some(ref dsn) => assert_eq!(dsn, "postgresql://anton:12345@localhost:5432/l09"),
            &None => assert!(false),
        }
    }

    #[test]
    fn no_postgres() {
        let _mutex_guard = ENV_MUTEX.lock().unwrap();

        env::remove_var("POSTGRES_DSN");
        match Config::new().get_postgres() {
            &Some(ref _dsn) => {
                assert!(false);
            }
            &None => assert!(true),
        }
    }
}
