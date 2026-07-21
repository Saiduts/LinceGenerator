#!/bin/bash
# Script de cross-compilación y despliegue generado por Lince Generator.
set -e

BINARY="monitor_lluvia_mhrd"
TARGET="aarch64-unknown-linux-gnu"
PI_USER="${PI_USER:-pi}"
PI_HOST="${PI_HOST:-raspberrypi.local}"
PI_DIR="${PI_DIR:-/home/pi}"

echo "==> Compilando para $TARGET..."
cargo build --release --target "$TARGET"

BINARY_PATH="target/$TARGET/release/$BINARY"
echo "==> Binario generado: $BINARY_PATH"

if [ -n "$PI_HOST" ]; then
    echo "==> Copiando binario y config.toml a $PI_USER@$PI_HOST:$PI_DIR ..."
    scp "$BINARY_PATH" config.toml "$PI_USER@$PI_HOST:$PI_DIR/"
    echo "==> Listo. Ejecuta en la Pi:"
    echo "    ssh $PI_USER@$PI_HOST 'chmod +x $PI_DIR/$BINARY && cd $PI_DIR && ./$BINARY'"
fi