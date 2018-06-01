/// Struct for one query monitoring result data
pub struct Diff {
    pub db_id: u32,
    pub query: String,
    pub calls: u64,
    pub mean_time: f64,
}
