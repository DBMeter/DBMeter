BEGIN;

CREATE ROLE dbmeter WITH password '12345' login;
CREATE SCHEMA IF NOT EXISTS dbmeter;
GRANT usage ON Schema dbmeter TO dbmeter;

CREATE OR REPLACE FUNCTION dbmeter.pg_stat_statements() RETURNS TABLE(
    queryid bigint,
    query text,
    calls bigint,
    total_time double precision
) as
$$
    select queryid, query, calls, total_time FROM pg_stat_statements;
$$ language sql SECURITY DEFINER;

commit ;