# General Design

## Communication Model

> Assuming the remote TCP listener is [Puredata](https://github.com/pure-data/pure-data)

- `vibed` --> `pd`: Open Sound Control (over FUDI (over TCP))
- `vibed` <-> `vibe`: JSON Remote Procedure Call (over WebSocket)

```text
[Pd] ───<OSC:FUDI:TCP|─── [VibeD] ───<JSON-RPC:WS>─── [Vibe]
```

## Design Decisions

### `vibe` & `vibed` Communication

I used Rust Enum (tagged union) to represent server and client commands (requests
responses, actions), and send them across using WebSocket in JSON format, for quick
prototyping and simplicity (at lease at first, when the complexity is low).

This turns out to be a "not very good" decision as the complexity grows and time
goes, the number of commands quickly grows turning the handling function into a
hundreds long nightmare. It is also hard to keep commands synchronized between two
different languages (Rust & TypeScript, or JavaScript with linting if you prefer).

Probably should've used gRPC with ProtoBuf, but it's hard to tell if I could finish
the project in time with two new things to learn.

### `vibed` & "Remote" Communication

Not much to say, TCP determined by Puredata, and OSC is the easiest way for arbitrary
message and limitless flexibility. Also, using OSC over FUDI because OSC over TCP
Binary doesn't work with Pd's way of handling packets.

### Choosing Tauri

This app needs to be cross-platform because not all of my teammates is under Linux.
And what's good for quick prototyping, fast iteration and written in Rust unlike
the heavy "counterpart" Electron.JS? Tauri.
