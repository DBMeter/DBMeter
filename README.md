DBMeter
==============

Create docker image:

```bash
cargo build --release
docker build -t dbmeter .
```

Create and run container named for example `dbmeter` based on image `dbmeter`:

```bash
docker run  --name dbmeter \
    -e "DBMETER_DB_DSN=postgresql://anton:12345@localhost:5432/l09" \
    -d \
    dbmeter
```

See some logs:
```bash
docker logs dbmeter | tail
```

Stop container: 
```bash
docker container stop dbmeter
```

Run it again
```bash
docker container run \
 -e "DBMETER_DB_DSN=postgresql://anton:12345@localhost:5432/l09" \
 -d \
 dbmeter
```