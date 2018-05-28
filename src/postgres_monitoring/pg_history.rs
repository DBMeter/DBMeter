use postgres_monitoring::pg_stat::PgStat;
use std::collections::HashMap;
use postgres_monitoring::monitoring_output::MonitoringOutput;

pub struct PgHistory {
    old_pg_stat: HashMap<i64, PgStat>,
}

impl PgHistory {
    pub fn new() -> PgHistory {
        PgHistory {
            old_pg_stat: HashMap::new(),
        }
    }

    pub fn save_stat_and_get_diff(&mut self, pg_stat: PgStat) -> MonitoringOutput {
        let mut diff = MonitoringOutput {
            query: pg_stat.query.clone(),
            calls: 0,
            mean_time: 0f64,
        };

        if let Some(old_stat) = self.old_pg_stat.get(&pg_stat.queryid) {
            diff.calls = pg_stat.calls - old_stat.calls;
            diff.mean_time = (pg_stat.calls - old_stat.calls) as f64
                / (pg_stat.total_time - old_stat.total_time);
        }

        self.old_pg_stat.insert(pg_stat.queryid, pg_stat);

        diff
    }
}
