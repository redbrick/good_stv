#!/usr/bin/env bash

set -e

until pg_isready -h db; do
	echo "."
	sleep 1
done

diesel setup
cargo watch -x 'run --bin good_stv_server'
