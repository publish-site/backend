#!/bin/sh

set -e 
curl -f -k https://127.0.0.1/ || exit 1
curl -f http://127.0.0.1:7878 || exit 1