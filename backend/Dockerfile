FROM rust:1.69 as build

RUN USER=root cargo new --bin backend
WORKDIR /backend

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src

RUN rm ./target/release/deps/backend*
RUN cargo build --release



FROM rust:1.69-slim-buster

COPY --from=build /backend/target/release/backend .

CMD ["./backend"]