# Dev

Install sqlx

cargo install sqlx-cli --version='~0.8' --no-default-features --features native-tls, postgres

Start Postgres
`./scripts/init_db.sh`


# Migration
sqlx migrate add create_subscriptions_table
sqlx migrate run

# Inspect data via psql
```bash
export DB_PORT=5432
export SUPERUSER=postgres
export SUPERUSER_PWD=password
export APP_DB_NAME=newsletter

psql postgres://${SUPERUSER}:${SUPERUSER_PWD}@localhost:${DB_PORT}/${APP_DB_NAME}
```
or alternativly use `psql -h localhost -p $DB_PORT -U $SUPERUSER -d $APP_DB_NAME`

In psql one can use `\dt` to list tables, and `\d subscriptions`.****