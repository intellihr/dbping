# dbwait

## Simple tool to wait until dbs are alive

### Usages

```!bash
    dbwait <db_urls>... --timeout <timeout>
```

The following command wait until both `db1` and `db2` are available (with `10s` timeout)

```
  dbwait -t 10 postgresql://db1:5432 postgresql://db2:5432
```
