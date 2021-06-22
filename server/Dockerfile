FROM rust as builder
WORKDIR /usr/src/dicebag-server
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
COPY --from=builder /usr/local/cargo/bin/dicebag-server /usr/local/bin/dicebag-server
EXPOSE 8000
CMD ["dicebag-server"]
