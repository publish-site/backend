#!/bin/bash

cat <<'EOF' > /etc/nginx/conf.d/deployment-proxy.conf
server {
    listen 80;
    server_name localhost.rvid.eu;

    location / {
        proxy_pass http://127.0.0.1:7878;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }
}
EOF

nginx -g "daemon off;" & backend
