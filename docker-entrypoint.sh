#!/bin/bash

if [ -n "$SSH" ]; then
  ssh-keygen -A
  echo "Port 2222
Protocol 2
PermitRootLogin yes
PasswordAuthentication no
KbdInteractiveAuthentication no
ChallengeResponseAuthentication no
PubkeyAuthentication yes
SyslogFacility AUTH
LogLevel VERBOSE" > /etc/ssh/sshd_config
  echo "$SSH" > /root/.ssh/authorized_keys
  chmod 600 /root/.ssh/authorized_keys
else
  echo "No SSH key found, exiting"
  exit 1
fi

cleanup() {
  pkill -TERM nginx
  pkill -TERM php-fpm84
  pkill -TERM sshd
  exit 0
}

trap cleanup SIGTERM SIGINT SIGKILL SIGQUIT

if [ "$PHP" = "true" ]; then
  export EXTRA="$EXTRA
  location ~ \.php$ {
    include fastcgi_params;
    fastcgi_pass 127.0.0.1:9000;
    fastcgi_index index.php;
    fastcgi_param SCRIPT_FILENAME \$document_root\$fastcgi_script_name;
  }"
  php-fpm84 -F &
fi

/usr/sbin/sshd -D -e &

envsubst "\$LOCATION \$EXTRA" < /config.conf > /etc/nginx/conf.d/config.conf
mkdir -p /etc/nginx/ssl

nginx -g "daemon off;"
pkill -TERM -P $$
exit 1