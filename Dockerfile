# build app 
FROM rust as builder
WORKDIR /app
COPY Cargo.toml ./
COPY src ./src
RUN cargo build --release 

FROM nginx:stable
RUN rm -f /etc/nginx/conf.d/default.conf
RUN mkdir /var/www/html -p
RUN chown -R nginx:nginx /var/www

COPY --from=builder /app/target/release/backend /usr/local/bin/backend
COPY docker-entrypoint.sh /
COPY config.conf /etc/nginx/conf.d 
ENV WEB_PATH=/var/www/html

ENTRYPOINT bash /docker-entrypoint.sh
