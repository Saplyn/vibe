# State & Storage

## Server State

> Runtime state

- Heartbeat
- Play position

## Server Storage

> Persistent data

```rust
struct Project {
  meta: Meta,
  patterns: HashMap<String, Pattern>,
  tracks: HashMap<String, Track>,
}

struct Pattern {
  page_count: u32,
  midi_path: String,
  midi_codes: Vec<Option<u8>>,
  messages: Vec<Message>
}

struct Track {
  active: bool,
  patterns: Vec<Pattern>,
}

struct Message {
  payload: OscMessage,
  active: Vec<bool>,
}

struct Meta {
  name: String,
  target_addr: String, // addr & port
}
```

## Client State

> Runtime state

## Client Storage

> Persistent data

- Server address (IP address, Port)
