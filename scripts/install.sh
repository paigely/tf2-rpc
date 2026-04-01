#!/usr/bin/env bash

if [[ $EUID -ne 0 ]]; then
	echo "run this script as root"
	exit 2
fi

set -e
set -o pipefail

install -Dm755 "target/release/tf2-rpc" "/usr/bin/tf2-rpc"
install -Dm644 "tf2-rpc.service" "/etc/systemd/user/tf2-rpc.service"

systemctl --user daemon-reload
systemctl --user enable --now tf2-rpc

echo "installed"
echo "remember to put -condebug in tf2's launch options"
