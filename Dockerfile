FROM rust:1.63.0-alpine as builder
WORKDIR /usr/src/myapp
COPY . .
RUN mkdir -p /root-layer/usr/bin && apk add build-base && cargo build --release && cp /usr/src/myapp/target/release/natpmp-setup /root-layer/usr/bin/
 
COPY root/ /root-layer/

# Final Docker Mods layer
FROM scratch
COPY --from=builder /root-layer/ /