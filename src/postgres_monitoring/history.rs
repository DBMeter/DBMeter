use postgres_monitoring::query_stat_snapshot::QueryStatSnapshot;
use std::collections::HashMap;
use postgres_monitoring::query_monitoring_output::QueryMonitoringOutput;

pub struct History {
    old_pg_stat: HashMap<i64, QueryStatSnapshot>,
}

impl History {
    pub fn new() -> History {
        History {
            old_pg_stat: HashMap::new(),
        }
    }

    pub fn save_stat_and_get_diff(
        &mut self,
        query_snapshot: QueryStatSnapshot,
    ) -> QueryMonitoringOutput {
        let mut diff = QueryMonitoringOutput {
            query: query_snapshot.query.clone(),
            calls: 0,
            mean_time: 0f64,
        };

        if let Some(old_stat) = self.old_pg_stat.get(&query_snapshot.queryid) {
            diff.calls = query_snapshot.calls - old_stat.calls;
            diff.mean_time = (query_snapshot.calls - old_stat.calls) as f64
                / (query_snapshot.total_time - old_stat.total_time);
        }

        self.old_pg_stat
            .insert(query_snapshot.queryid, query_snapshot);

        diff
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn save_stat_and_get_diff() {
        let query_snapshot1 = QueryStatSnapshot {
            queryid: 1,
            query: "SELECT 1".to_string(),
            calls: 5,
            total_time: 10f64,
        };

        let query_snapshot2 = QueryStatSnapshot {
            queryid: 1,
            query: "SELECT 1".to_string(),
            calls: 7,
            total_time: 20f64,
        };

        let mut history = History::new();
        history.save_stat_and_get_diff(query_snapshot1);
        let output: QueryMonitoringOutput = history.save_stat_and_get_diff(query_snapshot2);
        assert_eq!("SELECT 1", output.query);
        assert_eq!(2, output.calls);
        assert!(0.2f64 - 0.001 <= output.mean_time && output.mean_time <= 0.2f64 + 0.001);
    }

}
