use monitoring::diff::Diff;
use monitoring::snapshot::Snapshot;
use std::collections::HashMap;

pub struct History {
    storage: HashMap<String, Snapshot>,
}

impl History {
    pub fn new() -> History {
        History {
            storage: HashMap::new(),
        }
    }

    pub fn save_stat_and_get_diff(&mut self, query_snapshot: Snapshot) -> Diff {
        let snapshot_id = self.make_storage_id(&query_snapshot);
        let query = query_snapshot.query.clone();

        let mut diff = Diff {
            query: query.clone(),
            calls: 0,
            mean_time: 0.0,
        };

        if let Some(old_stat) = self.storage.get(&snapshot_id) {
            diff.calls = query_snapshot.calls - old_stat.calls;
            let mean_time = diff.calls as f64 / (query_snapshot.total_time - old_stat.total_time);
            diff.mean_time = if mean_time.is_nan() { 0.0 } else { mean_time }
        }

        self.storage.insert(snapshot_id.clone(), query_snapshot);
        diff
    }

    fn make_storage_id(&mut self, snapshot: &Snapshot) -> String {
        format!("{}{}", snapshot.dbid, snapshot.query)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use drivers::postgres::types;

    #[test]
    fn diff_calls_and_mean_time_is_corrected() {
        let mut query_snapshot1 = self::get_base_snapshot();
        query_snapshot1.calls = 5;
        query_snapshot1.total_time = 10 as types::DoublePrecision;

        let mut query_snapshot2 = self::get_base_snapshot();
        query_snapshot2.calls = 7;
        query_snapshot2.total_time = 20 as types::DoublePrecision;

        let mut history = History::new();
        history.save_stat_and_get_diff(query_snapshot1);

        let diff: Diff = history.save_stat_and_get_diff(query_snapshot2);

        assert!(0.2f64 - 0.001 <= diff.mean_time && diff.mean_time <= 0.2f64 + 0.001);
        assert_eq!(2, diff.calls);
    }

    #[test]
    fn identical_queries_from_different_db_will_not_be_compatible() {
        let mut query_snapshot1 = self::get_base_snapshot();
        query_snapshot1.dbid = 1;
        query_snapshot1.query = String::from("SELECT 1");
        query_snapshot1.calls = 5;
        query_snapshot1.total_time = 10 as types::DoublePrecision;

        let mut query_snapshot2 = self::get_base_snapshot();
        query_snapshot2.dbid = 2;
        query_snapshot2.query = String::from("SELECT 1");
        query_snapshot2.calls = 7;
        query_snapshot2.total_time = 20 as types::DoublePrecision;

        let mut history = History::new();
        history.save_stat_and_get_diff(query_snapshot1);

        let diff: Diff = history.save_stat_and_get_diff(query_snapshot2);

        assert_eq!(diff.calls, 0);
        assert_eq!(diff.mean_time, 0.0);
    }

    #[test]
    fn full_identical_queries_will_not_be_diff() {
        let mut query_snapshot1 = self::get_base_snapshot();
        query_snapshot1.dbid = 1;
        query_snapshot1.query = String::from("SELECT 1");
        query_snapshot1.calls = 5;
        query_snapshot1.total_time = 10 as types::DoublePrecision;

        let mut query_snapshot2 = self::get_base_snapshot();
        query_snapshot2.dbid = 1;
        query_snapshot2.query = String::from("SELECT 1");
        query_snapshot2.calls = 5;
        query_snapshot2.total_time = 10 as types::DoublePrecision;

        let mut history = History::new();
        history.save_stat_and_get_diff(query_snapshot1);

        let diff: Diff = history.save_stat_and_get_diff(query_snapshot2);

        println!("{}", diff.mean_time);

        assert_eq!(diff.calls, 0);
        assert_eq!(diff.mean_time, 0.0);
    }

    #[test]
    fn save_stat_and_get_diff() {
        let snapshot = self::get_base_snapshot();
        let mut history = History::new();
        let _diff: Diff = history.save_stat_and_get_diff(snapshot);
    }

    fn get_base_snapshot() -> Snapshot {
        Snapshot {
            userid: 1,
            dbid: 1,
            queryid: 1,
            query: "".to_string(),
            calls: 5,
            total_time: 10 as types::DoublePrecision,
            rows: 1,
            shared_blks_hit: 1,
            shared_blks_read: 1,
            shared_blks_dirtied: 1,
            shared_blks_written: 1,
            local_blks_hit: 1,
            local_blks_read: 1,
            local_blks_dirtied: 1,
            local_blks_written: 1,
            temp_blks_read: 1,
            temp_blks_written: 1,
            blk_read_time: 1 as types::DoublePrecision,
            blk_write_time: 1 as types::DoublePrecision,
        }
    }
}
