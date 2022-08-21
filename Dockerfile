FROM rust:1.61.0-alpine as builder
WORKDIR /root-layer/user/bin
COPY . .
RUN apk add build-base && cargo install --path /root-layer/user/bin
 
COPY root/ /root-layer/

# Final Docker Mods layer
FROM scratch
COPY --from=builder /root-layer/ /