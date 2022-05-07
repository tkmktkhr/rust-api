FROM rust:1.60 as develop-stage
WORKDIR /app
RUN cargo install cargo-watch
COPY . .

FROM develop-stage as build-stage
RUN cargo build --release

FROM rust:1.60-slim
COPY --from=build-stage /app/target/release/api .
EXPOSE 8080
CMD ["/usr/local/bin/api"]
