
FROM rust as builder

WORKDIR /app

RUN apt-get update
RUN apt-get install protobuf-compiler -y

COPY src src
COPY Cargo.toml Cargo.toml
COPY build.rs build.rs
COPY proto proto

RUN cargo build --release


FROM ubuntu

WORKDIR /app
COPY --from=builder /app/target/release/main .

CMD [ "./main" ]