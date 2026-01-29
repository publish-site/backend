# build app 
FROM rust as builder
WORKDIR /app
COPY Cargo.toml ./
COPY src ./src
RUN cargo build --release 

FROM nginx:stable

COPY --from=builder /app/target/release/backend /usr/local/bin/backend
COPY docker-entrypoint.sh / 
EXPOSE 80

ENTRYPOINT bash /docker-entrypoint.sh
