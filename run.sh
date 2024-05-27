#!/bin/sh

set -e

if [ -f .env ]; then 
	export $(grep -v '#' .env | xargs)
else
	echo ".env file not found!"
	exit 1
fi

cargo b --release
sudo setcap cap_net_admin=eip "$CARGO_TARGET_DIR"/release/rfc793
"$CARGO_TARGET_DIR"/release/rfc793 &
RUST_PROCESS_ID=$!
sudo ip addr add 192.168.0.1/24 dev tun0
sudo ip link set up dev tun0
wait $RUST_PROCESS_ID
