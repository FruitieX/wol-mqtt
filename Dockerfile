FROM gcr.io/distroless/static@sha256:28efbe90d0b2f2a3ee465cc5b44f3f2cf5533514cf4d51447a977a5dc8e526d0
COPY target/x86_64-unknown-linux-musl/release/wol-mqtt /usr/local/bin/wol-mqtt
CMD ["wol-mqtt"]
