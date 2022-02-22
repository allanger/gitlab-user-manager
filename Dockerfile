FROM rust:1.58.1 as builder
WORKDIR /build
COPY . .
RUN cargo build --release --bin gum

FROM alpine:3.15 as runtime
COPY --from=builder /build/target/release/gum /usr/local/bin
ENTRYPOINT ["./usr/local/bin/gum"]
