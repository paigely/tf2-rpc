#!/usr/bin/env bash

if [[ $EUID -ne 0 ]]; then
	echo "run this script as root"
	exit 2
fi

set -e
set -o pipefail

rm /usr/bin/tf2-rpc
rm /etc/systemd/user/tf2-rpc.service

systemctl --user disable tf2-rpc
systemctl --user daemon-reload

echo "uninstalled"
