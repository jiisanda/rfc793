#!/bin/bash

set -e

if [ -f .env ]; then 
    export $(grep -v '#' .env | xargs)
else
    echo ".env file not found!"
    exit 1
fi

# Create tun0 interface if it doesn't exist
if ! ip link show tun0 > /dev/null 2>&1; then
    sudo ip tuntap add dev tun0 mode tun
fi

# Build the Rust project
cargo b --release

# Set capabilities for the binary
sudo setcap cap_net_admin=eip "$CARGO_TARGET_DIR"/release/rfc793

# Run the binary in the background
"$CARGO_TARGET_DIR"/release/rfc793 &
RUST_PROCESS_ID=$!

# Configure the tun0 interface
sudo ip addr add 192.168.0.1/24 dev tun0
sudo ip link set up dev tun0

# Trap to kill the process and clean up
trap "sudo ip link set down dev tun0; sudo ip tun-tap del dev tun0 mode tun; kill $RUST_PROCESS_ID" INT TERM

# Wait for the Rust process to exit
wait $RUST_PROCESS_ID
