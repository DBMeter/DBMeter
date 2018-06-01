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

    pub fn save_stat_and_get_diff(&mut self, current_snapshot: Snapshot) -> Diff {
        let storage_key = self.make_storage_key(&current_snapshot);
        let query = current_snapshot.query.clone();

        let mut diff = Diff {
            query: query.clone(),
            calls: 0,
            mean_time: 0.0,
        };

        if let Some(storage_snapshot) = self.storage.get(&storage_key) {
            diff.calls = (current_snapshot.calls - storage_snapshot.calls) as u64;
            let mean_time = diff.calls as f64 / (current_snapshot.total_time - storage_snapshot.total_time);
            diff.mean_time = if mean_time.is_nan() { 0.0 } else { mean_time }
        }

        self.storage.insert(storage_key.clone(), current_snapshot);
        diff
    }

    fn make_storage_key(&mut self, snapshot: &Snapshot) -> String {
        format!("{}{}", snapshot.dbid, snapshot.query)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn diff_calls_and_mean_time_is_corrected() {
        let mut query_snapshot1 = self::get_base_snapshot();
        query_snapshot1.calls = 5;
        query_snapshot1.total_time = 10.0;

        let mut query_snapshot2 = self::get_base_snapshot();
        query_snapshot2.calls = 7;
        query_snapshot2.total_time = 20.0;

        let mut history = History::new();
        history.save_stat_and_get_diff(query_snapshot1);

        let diff = history.save_stat_and_get_diff(query_snapshot2);

        assert!(0.2f64 - 0.001 <= diff.mean_time && diff.mean_time <= 0.2f64 + 0.001);
        assert_eq!(2, diff.calls);
    }

    #[test]
    fn identical_queries_from_different_db_will_not_be_compatible() {
        let mut query_snapshot1 = self::get_base_snapshot();
        query_snapshot1.dbid = 1;
        query_snapshot1.query = String::from("SELECT 1");
        query_snapshot1.calls = 5;
        query_snapshot1.total_time = 10.0;

        let mut query_snapshot2 = self::get_base_snapshot();
        query_snapshot2.dbid = 2;
        query_snapshot2.query = String::from("SELECT 1");
        query_snapshot2.calls = 7;
        query_snapshot2.total_time = 20.0;

        let mut history = History::new();
        history.save_stat_and_get_diff(query_snapshot1);

        let diff = history.save_stat_and_get_diff(query_snapshot2);

        assert_eq!(diff.calls, 0);
        assert_eq!(diff.mean_time, 0.0);
    }

    #[test]
    fn full_identical_queries_will_not_be_diff() {
        let mut query_snapshot1 = self::get_base_snapshot();
        query_snapshot1.dbid = 1;
        query_snapshot1.query = String::from("SELECT 1");
        query_snapshot1.calls = 5;
        query_snapshot1.total_time = 10.0;

        let mut query_snapshot2 = self::get_base_snapshot();
        query_snapshot2.dbid = 1;
        query_snapshot2.query = String::from("SELECT 1");
        query_snapshot2.calls = 5;
        query_snapshot2.total_time = 10.0;

        let mut history = History::new();
        history.save_stat_and_get_diff(query_snapshot1);

        let diff = history.save_stat_and_get_diff(query_snapshot2);

        assert_eq!(diff.calls, 0);
        assert_eq!(diff.mean_time, 0.0);
    }

    fn get_base_snapshot() -> Snapshot {
        Snapshot {
            userid: 1,
            dbid: 1,
            queryid: 1,
            query: "".to_string(),
            calls: 5,
            total_time: 10.0,
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
            blk_read_time: 1.0,
            blk_write_time: 1.0,
        }
    }
}
