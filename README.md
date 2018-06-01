DBMeter [![Build Status](https://travis-ci.org/DBMeter/DBMeter.svg?branch=master)](https://travis-ci.org/DBMeter/DBMeter)
=============

## Installation guide

1. Install postgresql
2. Enable [pg_stat_statements](https://www.postgresql.org/docs/current/static/pgstatstatements.html) module in postgres
3. Execute `install.sql` from postgres user (change credentials if needed):
```bash
cat install.sql | sudo -u postgres psql -d postgres
``` 

4. Setup environment variable (change credentials if needed)
```bash
export POSTGRES_DSN="postgresql://dbmeter:12345@localhost:5432/postgres"
```

5. Run 
```bash
cargo run
```

