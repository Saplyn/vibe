# HTTP Communication

## Client -> Server

- Add/Edit/Delete
  - **Patterns**
    - Midi (OSC-path, `Vec<Option<u8>>`)
    - Message (multiple OSC-payload & `Vec<bool>`)
  - **Tracks**
    - Patterns (`Vec<Patterns>`)

## Server -> Client

- Performance:
  - **Heartbeat status** (Playing? Position?)
  - **Track status** (Playing? Position?)
- Programming:
  - **Heartbeat status** (Playing? Position?)
  - **Pattern status** (Playing? Position?)

## APIs

Using only WebSocket for both directions, send objects as JSON payloads.

---

## Scrapped

```text
WS     /ws

POST   /control/play
POST   /control/pause
POST   /control/stop

POST   /control/message

GET    /store/tracks
GET    /store/tracks/{name}
DELETE /store/tracks/{name}
PATCH  /store/tracks/{name}
```
