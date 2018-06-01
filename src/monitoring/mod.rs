use db_drivers::elasticsearch::ElasticSearch;
use db_drivers::postgres::Connection;
use export::Export;
use monitoring::history::History;
use monitoring::snapshot::Snapshot;
use std::thread;
use std::time::Duration;

pub mod diff;
mod history;
mod snapshot;

pub fn start_monitoring(postgres_dsn: String) {
    let connection = Connection::connect(postgres_dsn);
    let mut history = History::new();
    let interval = 200u64;

    loop {
        thread::sleep(Duration::from_millis(interval));

        let rows = &connection
            .query(
                "SELECT 
                    userid, 
                    dbid, 
                    queryid, 
                    query, 
                    calls, 
                    total_time,
                    rows,
                    shared_blks_hit,
                    shared_blks_read,
                    shared_blks_dirtied,
                    shared_blks_written,
                    local_blks_hit,
                    local_blks_read,
                    local_blks_dirtied,
                    local_blks_written,
                    temp_blks_read,
                    temp_blks_written,
                    blk_read_time,
                    blk_write_time
                 FROM dbmeter.pg_stat_statements()",
                &[],
            )
            .expect(
                "Couldn't select from dbmeter.pg_stat_statements. \
                 Please make sure it's properly installed",
            );

        for row in rows {
            let query_snapshot = Snapshot::from_row(row);
            let diff = history.save_stat_and_get_diff(query_snapshot);
            if diff.calls > 0 {
                ElasticSearch::save(Export::new(diff));
            }
        }
    }
}
