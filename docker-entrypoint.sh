#!/bin/bash

if [ ! -z PORT ]; then export PORT=80; fi
if [ -z API_URL ]; then
  echo "You must specify an API_URL environment variable!"
  exit 1
fi

conf="server {
    listen ${PORT};
    server_name ${API_URL};

    location / {
        proxy_pass http://127.0.0.1:7878;
        proxy_set_header Host \$host;
        proxy_set_header X-Real-IP \$remote_addr;
        proxy_set_header X-Forwarded-For \$proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto \$scheme;
    }
}"
echo "$conf" > /etc/nginx/conf.d/deployment-proxy.conf

nginx -g "daemon off;" & backend
