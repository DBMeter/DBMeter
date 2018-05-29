/// struct for row of pg_stat_statements

use postgres::rows::Row;

pub struct QueryStatSnapshot {
    pub queryid: i64,  //	 	Internal hash code, computed from the statement's parse tree
    pub query: String, // 	Text of a representative statement
    pub calls: i64,    // 	Number of times executed
    pub total_time: f64, //	 	Total time spent in the statement, in milliseconds
}

impl QueryStatSnapshot {
    pub fn from_row(row: Row) -> QueryStatSnapshot {
        QueryStatSnapshot {
            queryid: row.get("queryid"),
            query: row.get("query"),
            calls: row.get("calls"),
            total_time: row.get("total_time"),
        }
    }
}
