
FROM rust as builder

WORKDIR /app

COPY src src
COPY Cargo.toml Cargo.toml

RUN cargo build --release


FROM ubuntu

WORKDIR /app
COPY --from=builder /app/target/release/simulated_annealing .

CMD [ "./simulated_annealing" ]