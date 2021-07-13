FROM rust:1.53 as builder
WORKDIR /usr/src/dicebag-server
COPY . .
RUN cargo install --path .

FROM debian:buster-slim as runner
COPY --from=builder /usr/local/cargo/bin/dicebag-server /usr/local/bin/dicebag-server
ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000
CMD ["dicebag-server"]
