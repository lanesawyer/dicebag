FROM rust:1.53 as builder
WORKDIR /usr/src/dicebag-server
COPY . .
RUN cargo install --path .

FROM debian:buster-slim as runner
RUN apt-get update && apt-get install -y \
        postgresql postgresql-contrib \
        && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/dicebag-server /usr/local/bin/dicebag-server
ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000
CMD ["dicebag-server"]
