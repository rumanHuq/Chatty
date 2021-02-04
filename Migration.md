```sh
  $ cargo install sqlx-cli --no-default-features --features postgres #1
  $ export DATABASE_URL="postgres://postgres@localhost/DB_NAME" #2
  $ sqlx database drop/create #Optional
  $ sqlx migrate add <FILE_NAME> #3
  # This will create migrations/<TIMESTAMP_FILE_NAME>.sql
  # put all the create table sql queries in the file
  $ sqlx migrate run #4
```
