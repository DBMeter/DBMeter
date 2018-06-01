use postgres::{Connection as BaseConnection, TlsMode};

pub struct Connection;

impl Connection {
    pub fn connect(postgres_dsn: String) -> BaseConnection {
        let connection = BaseConnection::connect(postgres_dsn, TlsMode::None);

        match &connection {
            &Ok(ref _result) => println!("Connected"),
            &Err(ref _error) => {
                panic!("Connection error: {}", _error);
            }
        }

        connection.unwrap()
    }
}
