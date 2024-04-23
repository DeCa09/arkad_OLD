#!/bin/ash

# Define variables
ARCH="aarch64"
OS="linux"
ZIG_VERSION="0.12.0"
ZIG_DOWNLOAD_URL="https://ziglang.org/download/${ZIG_VERSION}/zig-${OS}-${ARCH}-${ZIG_VERSION}.tar.xz"
INSTALL_DIR="/usr/local/zig/"

# PRE-Requisites
apk update && apk add musl-dev

# Download Zig tarball
wget "$ZIG_DOWNLOAD_URL"

# Extract the tarball
tar -xvf "zig-${OS}-${ARCH}-${ZIG_VERSION}.tar.xz"

# Move Zig binary to INSTALL_DIR
mv "zig-${OS}-${ARCH}-${ZIG_VERSION}" "$INSTALL_DIR"

# Add zig to PATH: TODO: klappt noch net
echo "export PATH=$PATH:/usr/local/zig/" >> /etc/profile

# Clean up the tarball
rm zig-linux-aarch64-0.12.0.tar.xz

# Display installation success message
echo "Zig version $ZIG_VERSION has been installed to $INSTALL_DIR"
