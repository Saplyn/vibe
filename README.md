# Vibe Kit

## Features

- [Live connectivity check](./assets/connectivity.mp4)
- [Track Composing & on-beat on-off](./assets/tracks.mp4)
- [Pattern based programming](./assets/patterns.mp4)
- [One-off events & numerical slider](./assets/controls.png)

## Using The App

1. Start the server `vibed`.
2. Start the client `vibe`, and connect to the server.
3. You may also want to start the remote OSC TCP listener.
4. Good to go!

## Developing

Ensure [Rust](https://www.rust-lang.org/) and [Bun](https://bun.sh/) are installed,
as well as have complete [the prerequisites for developing a Tauri app](https://tauri.app/start/prerequisites/).

```bash
# Develop the server: `vibed`
cargo run
# Develop the client: `vibe`
bunx tauri dev

# Building the server: `vibed`
cargo build
# Building the client: `vibe`
bun install
NO_STRIP=true bunx tauri build
```
