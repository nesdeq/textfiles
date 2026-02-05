# TEXTFILES.COM Browser

*Back when the internet was weird, wonderful, and made entirely of text.*

![screenshot](textfiles.png)

A terminal browser for [textfiles.com](http://textfiles.com) — Jason Scott's incredible archive of BBS-era text files. Anarchy cookbooks, hacker manifestos, game walkthroughs, conspiracy theories, and digital folklore from the 1980s-90s. Before social media. Before web 2.0. Before everything got sanitized.

This is how we used to browse.

## Build

```bash
git clone <repo>
cd textfiles
cargo build --release
./target/release/textfiles-browser
```

## Controls

- `j/k` or arrows — navigate
- `Enter` — open
- `Backspace` — go back
- `q` — quit

## Old School Mode

For the authentic 1990s BBS experience, use the included launcher scripts:

```bash
./run.sh   # POSIX shell
./run.fish # fish shell
```

These launch the browser in Alacritty with:
- 80x30 character grid (classic DOS text mode)
- 2x scaled Perfect DOS VGA 437 font
- Phosphor green on black color scheme

### Optional Dependencies

For old school mode:
- [Alacritty](https://alacritty.org/) — GPU-accelerated terminal
- [Perfect DOS VGA 437](https://github.com/CP437/PerfectDOSVGA437) — authentic DOS font

## Requirements

Rust 1.75+ and a mass case of nostalgia.

---

*"The files are out there."* — textfiles.com
