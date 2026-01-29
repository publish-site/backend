# build app 
FROM rust as builder
WORKDIR /app
COPY Cargo.toml ./
COPY src ./src
RUN cargo build --release 

FROM nginx:stable

COPY --from=builder /app/target/release/backend /usr/local/bin/backend

EXPOSE 8080

CMD ["app"]

