FROM rust:1.58.1 as builder
RUN mkdir -p /src
WORKDIR /src
COPY ./ .
RUN cargo build --release

FROM ubuntu:latest
COPY --from=builder /src/target/release/gum /bin/gum
WORKDIR /config
ENTRYPOINT ["/bin/gum"]