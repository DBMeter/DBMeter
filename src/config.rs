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
        } else {
            panic!("ERROR: Setup `POSTGRES_DSN` environment variable. Read more in ./README.md");
        }

        config
    }

    pub fn get_postgres(&self) -> &Option<String> {
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
        let _mutex_guard = ENV_MUTEX.lock();
        let env_dsn = "postgresql://dbmeter:12345@localhost:5432/l09";

        env::set_var("POSTGRES_DSN", env_dsn);

        if let Some(config_dsn) = Config::new().get_postgres() {
            assert_eq!(config_dsn, env_dsn)
        }
    }

    #[test]
    #[should_panic]
    fn no_postgres() {
        let _mutex_guard = ENV_MUTEX.lock();
        env::remove_var("POSTGRES_DSN");
        Config::new().get_postgres();
    }
}
