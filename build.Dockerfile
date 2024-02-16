FROM rust:bullseye as builder

WORKDIR /repos

COPY Cargo.lock Cargo.toml ./

# copy in dummy application so that we can build dependencies
# NOTE: this is to avoid rebuilding dependencies if source files change
COPY scripts/main.rs src/
RUN cargo build --release

# copy in actual application and build
COPY src src
RUN cargo build --release

FROM debian:bullseye-slim as runner

WORKDIR /app

RUN apt-get update
RUN apt-get install -y libcurl4

COPY --from=builder /repos/target/release/zircon-api .
COPY config config

CMD ["./zircon-api"]
