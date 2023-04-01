FROM rust:1.68 AS build
WORKDIR /app
COPY . .
RUN rustup target add x86_64-unknown-linux-musl
RUN cargo build --release --target x86_64-unknown-linux-musl

FROM alpine:3.17
WORKDIR /app
COPY --from=build /app/target/x86_64-unknown-linux-musl/release/tagebuch .
ENV RUST_LOG=info

CMD ["/app/tagebuch"]
