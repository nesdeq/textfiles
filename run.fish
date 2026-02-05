#!/usr/bin/env fish

set -l script_dir (dirname (status filename))
set -l binary "$script_dir/target/release/textfiles-browser"

# Build if needed
if not test -f $binary
    cargo build --release --manifest-path="$script_dir/Cargo.toml"
end

alacritty \
    -o 'window.dimensions.columns=80' \
    -o 'window.dimensions.lines=30' \
    -o 'font.normal.family="Perfect DOS VGA 437"' \
    -o 'font.size=32' \
    -o 'colors.primary.background="#000a00"' \
    -o 'colors.primary.foreground="#39ff14"' \
    --title "TEXTFILES.COM" \
    -e $binary
