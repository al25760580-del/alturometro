# alturometro

Measure your height in bananas, Big Macs, rubber ducks and other units nobody asked for.

## What is this

Two programs that do the same thing but for different people:

- **CLI** (`alturometro-meme/`) — runs in a terminal. Static binary, zero dependencies.
- **GUI** (`alturometro-gui/`) — windowed app with Slint. Modern look, software rendered.

## Quick start

Grab a binary from `bin/` or the releases page.

```bash
# CLI
./alturometro-linux-x86_64 1.81

# GUI (Linux)
./alturometro-gui-linux-x86_64
```

Type your height in meters. Use a dot or comma for decimals.

## Build from source

You need Rust 1.85+ (edition 2024):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### CLI

```bash
cd alturometro-meme
rustup target add x86_64-unknown-linux-musl
cargo build --release --target x86_64-unknown-linux-musl
```

Output is a fully static binary. Copy it anywhere, it works.

### GUI

```bash
cd alturometro-gui
cargo build --release
```

Slint compiles itself. No external GUI libraries needed.

## CI

GitHub Actions builds automatically for:

| Target | CLI | GUI |
|--------|-----|-----|
| Linux x86_64 (musl static) | Yes | Yes |
| Linux aarch64 (musl static) | Yes | - |
| macOS x86_64 | Yes | - |
| macOS aarch64 (Apple Silicon) | Yes | Yes |
| Windows x86_64 | Yes | Yes |

Push to `main` and the workflow runs. Artifacts appear in the Actions tab.

## Compatibility

The CLI is a static musl binary. It has zero runtime dependencies.
It runs on any Linux kernel 3.2+ on x86_64. No libc, no .so files.

The GUI needs fontconfig and freetype on Linux (any desktop has them).
On Windows and macOS it works out of the box.

### What about Windows XP / really old stuff

Rust dropped Windows XP support in 2019. The minimum is Windows 10.
If you need XP, use the CLI through WINE or compile with the `thunk`
crate and an older Rust nightly.

Pentium 4 is fine for modern Rust (it has SSE2). Anything older than
Pentium III needs a `i586` target and loses the GUI.

## Where the numbers come from

Every measurement has a source.

| Unit | Size | Source |
|------|------|--------|
| Banana | 18 cm | USDA: Cavendish medium 17-20 cm |
| Apple | 7.5 cm diameter | Wikipedia (Malus domestica): 7.0-8.3 cm |
| Big Mac | 9 cm tall | WhatsNeue (2018): 6.9 cm measured with caliper |
| Rubber duck | 10 cm | Commercial standard: 4 inches |
| Great white shark | 4.5 m | CSULB Shark Lab: adults average 4.3-4.5 m |
| Giraffe | 5 m | Guinness / San Diego Zoo: males 4.6-5.5 m |
| New Routemaster | 11.1 m | TfL / Wikipedia: London double-decker |
| Blue whale | 24 m | Wikipedia / Monterey Bay Aquarium: 24-25 m |
| Football field | 105 m | FIFA regulations |
| Eiffel Tower | 330 m | Official: 330 m with antenna |

## License

WTFPL v2. Do what you want.

Slint is GPL-3.0. Your code is WTFPL. The combined binary ships under
GPL-3.0 terms for the Slint parts. Read and share freely.