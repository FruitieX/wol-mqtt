FROM gcr.io/distroless/static@sha256:ce46866b3a5170db3b49364900fb3168dc0833dfb46c26da5c77f22abb01d8c3
COPY target/x86_64-unknown-linux-musl/release/wol-mqtt /usr/local/bin/wol-mqtt
CMD ["wol-mqtt"]
