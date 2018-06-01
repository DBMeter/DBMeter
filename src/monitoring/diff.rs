/// Struct for one query monitoring result data
pub struct Diff {
    pub query: String,
    pub calls: u64,
    pub mean_time: f64,
}
