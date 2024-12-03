FROM rust:1.75-alpine as builder
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release

FROM alpine:latest
COPY --from=builder /usr/src/app/target/release/your_binary /usr/local/bin/
ENV PORT=8080
EXPOSE ${PORT}
CMD ["cargo run --bin server"]