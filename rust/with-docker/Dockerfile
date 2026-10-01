# https://docs.docker.com/guides/rust/

FROM docker.io/library/rust:alpine AS builder
WORKDIR /app
COPY . ./
RUN cargo build --release

FROM scratch
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/
# COPY --from=builder /usr/share/zoneinfo /usr/share/zoneinfo
# COPY --from=builder /etc/mime.types /etc/mime.types
COPY --from=builder /app/target/release/with-docker /bin/with-docker
# EXPOSE 8080
CMD ["/bin/with-docker"]
