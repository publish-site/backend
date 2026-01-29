#!/bin/bash

export PORT="${PORT:-80}"

if [ -z "$API_URL" ]; then
  echo "Error: API_URL environment variable is required."
  exit 1
fi

mkdir -p /etc/nginx/ssl

# ENV vars set when starting docker.
if [ -n "$FULLCHAIN" ]; then
  echo "$FULLCHAIN" | base64 -d > /etc/nginx/ssl/fullchain.pem
  echo "Fullchain certificate set."
fi

if [ -n "$PRIVKEY" ]; then
  echo "$PRIVKEY" | base64 -d > /etc/nginx/ssl/privkey.pem
  echo "Private key set."
fi

# Generate certs
#if [ ! -f /etc/nginx/ssl/fullchain.pem ] && [ ! -f /etc/nginx/ssl/privkey.pem ]; then
#fi

conf="server {
    ssl_certificate /etc/nginx/ssl/fullchain.pem;
    ssl_certificate_key /etc/nginx/ssl/privkey.pem;
    listen $PORT ssl;
    server_name ${API_URL};

    location / {
        proxy_pass http://127.0.0.1:${API_PORT:-7878};
        proxy_set_header Host \$host;
        proxy_set_header X-Real-IP \$remote_addr;
        proxy_set_header X-Forwarded-For \$proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto \$scheme;
    }
}"

echo "$conf" > /etc/nginx/conf.d/deployment-proxy.conf

backend &
exec nginx -g "daemon off;"
