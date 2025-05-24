# Vibe Daemon

## Architecture

The server is split into 4 main perts:

- `ticker`: Generates heartbeat ticks for music pace controlling
- `communicator`: Connects & communicates with remote TCP server
- `controller`: Middleman between above two, handles which message to send
- `handler`: Handles client request via WebSocket JSON-RPC

Different parts communicates with each other via `tokio` provided async channels,
while sharing the same central `Store` state.

## Execution Flow

```text
─[main]─┬────────────────────────[start:ws-server]─╌╌
        ├─[spawn:ticker]───────╌╌
        ├─[spawn:communicator]─╌╌
        └─[spawn:controller]───╌╌
```

## Design Decisions

### Choosing Rust & Axum

It's the language I'm most familiar with, and the library is ranked number 1 in
[lib.rs's HTTP Server section](https://lib.rs/web-programming/http-server).
For async programming, I enabled `full` on `tokio` simply because I'm too lazy
to turn on the feature one by one.

I personally think this is a great decision. Rust and it's ecosystem makes concurrent
programming fearless and so much easier. Refactoring is also a lot easier thanks
to `rust-analyzer`.

### Spawning So Many "Threads"

I fully understand that async `spawn()` is not the same as the one provided in
the standard library, but I'm going with the name "thread" to keep me sane.

Originally I did this because my brain is too weak to process everything in one
threads, so I have to split them up. Ticker needs to be on a separate thread
for me to stay sane. Controller and communicator were in the same thread but
were split apart due to me being unable to juggle so much information at the
same time.

This turns out to be a good decision, although not perfect. Splitting the
responsibility makes adding new functionalities a breeze. Even though
inter-threads communication is kind of messy with lots of channels here and
there, but it's a complexity that I can handle.
