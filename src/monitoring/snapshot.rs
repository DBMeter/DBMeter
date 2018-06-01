use drivers::postgres;
use postgres::rows::Row;

/// struct for row of pg_stat_statements
pub struct Snapshot {
    pub userid: postgres::types::Oid, // OID of user who executed the statement
    pub dbid: postgres::types::Oid,   // OID of database in which the statement was executed
    pub queryid: postgres::types::Bigint, // Internal hash code, computed from the statement's parse tree
    pub query: postgres::types::Text,     // Text of a representative statement
    pub calls: postgres::types::Bigint,   // Number of times executed
    pub total_time: postgres::types::DoublePrecision, // Total time spent in the statement, in milliseconds
    pub rows: postgres::types::Bigint, // Total number of rows retrieved or affected by the statement
    pub shared_blks_hit: postgres::types::Bigint, // Total number of shared block cache hits by the statement
    pub shared_blks_read: postgres::types::Bigint, // Total number of shared blocks read by the statement
    pub shared_blks_dirtied: postgres::types::Bigint, // Total number of shared blocks dirtied by the statement
    pub shared_blks_written: postgres::types::Bigint, // Total number of shared blocks written by the statement
    pub local_blks_hit: postgres::types::Bigint, // Total number of local block cache hits by the statement
    pub local_blks_read: postgres::types::Bigint, // Total number of local blocks read by the statement
    pub local_blks_dirtied: postgres::types::Bigint, // Total number of local blocks dirtied by the statement
    pub local_blks_written: postgres::types::Bigint, // Total number of local blocks written by the statement
    pub temp_blks_read: postgres::types::Bigint, // Total number of temp blocks read by the statement
    pub temp_blks_written: postgres::types::Bigint, // Total number of temp blocks written by the statement
    pub blk_read_time: postgres::types::DoublePrecision, // Total time the statement spent reading blocks, in milliseconds (if track_io_timing is enabled, otherwise zero)
    pub blk_write_time: postgres::types::DoublePrecision, // Total time the statement spent writing blocks, in milliseconds (if track_io_timing is enabled, otherwise zero)
}

impl Snapshot {
    pub fn from_row(row: Row) -> Snapshot {
        Snapshot {
            userid: row.get("userid"),
            dbid: row.get("dbid"),
            queryid: row.get("queryid"),
            query: row.get("query"),
            calls: row.get("calls"),
            total_time: row.get("total_time"),
            rows: row.get("rows"),
            shared_blks_hit: row.get("shared_blks_hit"),
            shared_blks_read: row.get("shared_blks_read"),
            shared_blks_dirtied: row.get("shared_blks_dirtied"),
            shared_blks_written: row.get("shared_blks_written"),
            local_blks_hit: row.get("local_blks_hit"),
            local_blks_read: row.get("local_blks_read"),
            local_blks_dirtied: row.get("local_blks_dirtied"),
            local_blks_written: row.get("local_blks_written"),
            temp_blks_read: row.get("temp_blks_read"),
            temp_blks_written: row.get("temp_blks_written"),
            blk_read_time: row.get("blk_read_time"),
            blk_write_time: row.get("blk_write_time"),
        }
    }
}
