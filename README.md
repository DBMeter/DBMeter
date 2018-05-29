DBMeter [![Build Status](https://travis-ci.org/DBMeter/DBMeter.svg?branch=master)](https://travis-ci.org/DBMeter/DBMeter)
=============

First execute `install.sql` from postgres user
(change password in file):

```bash
cat install.sql | sudo -u postgres psql -d postgres
``` 

then build project:

```bash
cargo build --release
```

Setup environment variable
```bash
export POSTGRES_DSN="postgresql://dbmeter:12345@localhost:5432/postgres"
```

and run 
```bash
target/release/dbmeter
```

