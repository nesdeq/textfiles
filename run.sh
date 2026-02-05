#!/bin/sh

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
BINARY="$SCRIPT_DIR/target/release/textfiles-browser"

# Build if needed
if [ ! -f "$BINARY" ]; then
    cargo build --release --manifest-path="$SCRIPT_DIR/Cargo.toml"
fi

alacritty \
    -o 'window.dimensions.columns=80' \
    -o 'window.dimensions.lines=30' \
    -o 'font.normal.family="Perfect DOS VGA 437"' \
    -o 'font.size=32' \
    -o 'colors.primary.background="#000a00"' \
    -o 'colors.primary.foreground="#39ff14"' \
    --title "TEXTFILES.COM" \
    -e "$BINARY"
