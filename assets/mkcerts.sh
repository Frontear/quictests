#!/usr/bin/env bash

set -o errexit
set -o nounset
set -o pipefail

if ! which openssl &> /dev/null; then
  echo "openssl not on PATH, cannot proceed" >& 2
  exit 1
fi

cwd=$(realpath $(dirname "$0"))

openssl req \
  -new \
  -newkey ec \
  -pkeyopt ec_paramgen_curve:prime256v1 \
  -x509 \
  -nodes \
  -out "$cwd/cert.pem" \
  -keyout "$cwd/key.pem" \
  -days 3650 \
  -config "$cwd/ca.cnf"
