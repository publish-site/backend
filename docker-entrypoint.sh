#!/bin/bash

export PORT="${PORT:-443}"

if [ -z "$API_URL" ]; then
  echo "Error: API_URL environment variable is required."
  exit 1
fi

eval "cat <<EOF
$(< /config.conf)
EOF" > /etc/nginx/conf.d/config.conf

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

if [ -n "$CLIENT_CA" ]; then
  echo "$CLIENT_CA" | base64 -d > /etc/nginx/ssl/ca.pem
fi

# Generate certs
#if [ ! -f /etc/nginx/ssl/fullchain.pem ] && [ ! -f /etc/nginx/ssl/privkey.pem ]; then
#fi

su -c "backend &" nginx
exec nginx -g "daemon off;"
