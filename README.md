# rust-api
Rust api server

## Format

`cargo fmt`

## Build

`cargo build`

## Test

`cargo test`

## Run

### Local

`cargo run`
or
`cargo run watch -x`

### Docker Container

`docker-compose up api`

#### Docker Container Login

`docker exec -it <CONTAINER ID> sh`

## DB

### MySQL

Access from local pc.

`docker exec -it <container id> /bin/sh`

### Diesel

`$ diesel setup`
Creating migrations directory at: /Users/<user-name>/<path>/<repo-name>/migrations

#### migration run

Generate the migration point
`diesel migration generate <MIGRATION_NAME> --format <MIGRATION_FORMAT>`

Execute the newest up.sql
`diesel migration run`

Execute the newest down.sql
`diesel migration revert`

Execute the `down.sql` then `up.sql`
`diesel migration redo`
