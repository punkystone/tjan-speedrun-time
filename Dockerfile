
FROM rust:1.75-slim-bookworm as build
RUN apt update && apt install pkg-config libssl-dev -y
RUN USER=root cargo new --bin tjan_speedrun_time
WORKDIR /tjan_speedrun_time

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src

RUN rm ./target/release/deps/tjan_speedrun_time*
RUN cargo build --release



FROM rust:1.75-slim-bookworm

COPY --from=build /tjan_speedrun_time/target/release/tjan_speedrun_time .

CMD ["./tjan_speedrun_time"]
