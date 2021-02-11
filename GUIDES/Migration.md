## Click 👉 [sqlx-cli docs](https://github.com/launchbadge/sqlx/tree/master/sqlx-cli)

```bash
  $ cargo install sqlx-cli --no-default-features --features postgres #1
  $ export DATABASE_URL="postgres://user:password@localhost/DB_NAME" #2
  $ sqlx database drop/create #Optional
  $ sqlx migrate add <FILE_NAME> #3 # This will create migrations/<TIMESTAMP_FILE_NAME>.sql
  $ sqlx migrate run #4
```
