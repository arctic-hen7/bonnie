#!/bin/sh

# This script is designed for Docker usage only! It must be run as root, and will install Bonnie directly to `/bin`.

# Get the URL to the latest version of the musl binary
url=$(curl -s https://api.github.com/repos/arctic-hen7/bonnie/releases/latest | grep 'browser_' | cut -d\" -f4 | grep musl)

# Download the latest release of Bonnie for musl and put it in `/bin`
curl -L $url > /bin/bonnie
chmod +x /bin/bonnie
