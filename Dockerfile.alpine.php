# build app 
FROM rustlang/rust:nightly-alpine AS builder
WORKDIR /app
COPY Cargo.toml ./
COPY src ./src
RUN cargo build --release 
RUN apk add --no-cache upx
RUN upx --best --lzma target/release/backend

FROM nginx:alpine-slim
RUN apk --no-cache add \
        php84 \
        php84-ctype \
        php84-curl \
        php84-dom \
        php84-exif \
        php84-fileinfo \
        php84-fpm \
        php84-gd \
        php84-iconv \
        php84-intl \
        php84-json \
        php84-mbstring \
        php84-mysqli \
        php84-opcache \
        php84-openssl \
        php84-pecl-apcu \
        php84-pdo \
        php84-pdo_mysql \
        php84-pgsql \
        php84-phar \
        php84-session \
        php84-simplexml \
        php84-soap \
        php84-sodium \
        php84-sqlite3 \
        php84-tokenizer \
        php84-xml \
        php84-xmlreader \
        php84-zip \
        php84-zlib \
        tini \
        base64

RUN rm -f /etc/nginx/conf.d/default.conf
RUN mkdir /var/www/html -p
RUN chown -R nginx:nginx /var/www

ENV PHP=true

COPY --from=builder /app/target/release/backend /usr/local/bin/backend
COPY docker-entrypoint.sh /
COPY config.conf /config.conf
COPY www.conf /etc/php84/php-fpm.d/www.conf
ENV WEB_PATH=/var/www/html

ENTRYPOINT ["/sbin/tini", "--", "ash", "/docker-entrypoint.sh"]
