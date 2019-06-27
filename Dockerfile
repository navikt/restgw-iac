FROM alpine

ENV RUST_LOG info

COPY target/x86_64-unknown-linux-musl/release/restgw-iac /usr/bin/restgw-iac
COPY configuration.json configuration.json
