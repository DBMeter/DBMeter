use postgres::{Connection, TlsMode};
use std::thread;
use std::time::Duration;
use postgres_monitoring::pg_stat::PgStat;
use postgres_monitoring::pg_history::PgHistory;

mod pg_history;
mod pg_stat;
mod monitoring_output;

pub fn start_monitoring(postgres_url: String) {
    println!("postgres url: {}", postgres_url);

    let result = Connection::connect(postgres_url, TlsMode::None);
    match &result {
        &Ok(ref _result) => println!("Connected"),
        &Err(ref _error) => {
            println!("Connection error: {}", _error);
            return;
        }
    }

    let connection = result.unwrap();

    let mut pg_history = PgHistory::new();

    let interval = 1u64;

    loop {
        thread::sleep(Duration::from_secs(interval));

        let rows = &connection
            .query(
                "SELECT queryid, query, calls, total_time \
                 FROM dbmeter.pg_stat_statements()",
                &[],
            )
            .expect("ошибка");

        for row in rows {
            let pg_stat = PgStat::from_row(row);
            let monitoring_output = pg_history.save_stat_and_get_diff(pg_stat);
            if monitoring_output.calls > 0 {
                println!(
                    "query: {}, mean_time: {}",
                    monitoring_output.query, monitoring_output.mean_time
                );
            }
        }
    }
}
