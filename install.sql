BEGIN;

CREATE ROLE dbmeter WITH password '12345' login;
CREATE SCHEMA IF NOT EXISTS dbmeter;
GRANT usage ON Schema dbmeter TO dbmeter;

CREATE OR REPLACE FUNCTION dbmeter.pg_stat_statements() RETURNS TABLE(
    userid oid,
    dbid oid,
    queryid bigint,
    query text,
    calls bigint,
    total_time double precision,
    rows bigint,
    shared_blks_hit bigint,
    shared_blks_read bigint,
    shared_blks_dirtied bigint,
    shared_blks_written bigint,
    local_blks_hit bigint,
    local_blks_read bigint,
    local_blks_dirtied bigint,
    local_blks_written bigint,
    temp_blks_read bigint,
    temp_blks_written bigint,
    blk_read_time double precision,
    blk_write_time double precision
) as
$$
   SELECT 
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
    FROM pg_stat_statements
$$ language sql SECURITY DEFINER;

COMMIT;
