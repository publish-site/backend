#!/bin/bash

export PORT="${PORT:-443}"
export BODY_SIZE="${BODY_SIZE:-2000M}"

cleanup() {
  kill -TERM "$BACKEND_PID" "$NGINX_PID" 2>/dev/null
  wait "$BACKEND_PID" 2>/dev/null
  wait "$NGINX_PID" 2>/dev/null
  exit 0
}

trap cleanup SIGTERM SIGINT

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

backend &
BACKEND_PID=$!

nginx -g "daemon off;" &
NGINX_PID=$!

wait -n

kill -TERM "$BACKEND_PID" "$NGINX_PID" 2>/dev/null
wait "$BACKEND_PID" "$NGINX_PID" 2>/dev/null
exit 1