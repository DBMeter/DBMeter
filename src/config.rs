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

    pub fn get_postgres(self) -> Option<String> {
        self.postgres_dsn
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn get_postgres() {
        env::set_var(
            "POSTGRES_DSN",
            "postgresql://anton:12345@localhost:5432/l09",
        );
        match Config::new().get_postgres() {
            Some(_dsn) => assert!(true),
            None => assert!(false),
        }
        env::remove_var("POSTGRES_DSN");
    }

    #[test]
    fn no_postgres() {
        match Config::new().get_postgres() {
            Some(_dsn) => {
                println!("{}", _dsn);
                assert!(false);
            }
            None => assert!(true),
        }
    }
}
