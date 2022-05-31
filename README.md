# rust-api
Rust api server

## Format

`cargo fmt`

## Build

`cargo build`

## Run

### Local

`cargo run`

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