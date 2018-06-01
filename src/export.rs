use elastic::prelude::{Date, DefaultDateMapping};
use monitoring::diff::Diff;

#[derive(Debug, Serialize, Deserialize, ElasticType)]
pub struct Export {
    pub db_id: i32,
    pub query: String,
    pub diff_calls: i64,
    pub diff_mean_time: f64,
    pub date_time: Date<DefaultDateMapping>,
}

impl Export {
    pub fn new(diff: Diff) -> Export {
        Export {
            db_id: diff.db_id as i32,
            query: diff.query,
            diff_calls: diff.calls as i64,
            diff_mean_time: diff.mean_time,
            date_time: Date::now(),
        }
    }
}
