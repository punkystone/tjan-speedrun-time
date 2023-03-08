
FROM rust:1.65 as build

RUN USER=root cargo new --bin tjan_speedrun_time
WORKDIR /tjan_speedrun_time

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src

RUN rm ./target/release/deps/tjan_speedrun_time*
RUN cargo build --release



FROM rust:1.65-slim-buster

COPY --from=build /tjan_speedrun_time/target/release/tjan_speedrun_time .

CMD ["./tjan_speedrun_time"]
