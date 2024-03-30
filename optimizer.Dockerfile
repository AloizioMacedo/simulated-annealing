
FROM rust as builder

WORKDIR /app

COPY src src
COPY Cargo.toml Cargo.toml

RUN cargo build --release


FROM ubuntu

WORKDIR /app
COPY --from=builder /app/target/release/main .

CMD [ "./main" ]