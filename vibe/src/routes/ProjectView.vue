<template>
  <div>
    <Button>uwu</Button>
  </div>
</template>

<script setup lang="ts">
import { error, info } from "@tauri-apps/plugin-log";
import WebSocket from "@tauri-apps/plugin-websocket";

WebSocket.connect("ws://127.0.0.1:3000")
  .then((ws) => {
    info("connected");

    ws.send("Hello World!").then(() => {
      info("send hello world");
    });

    ws.addListener((msg) => {
      info("Received Message:" + msg);
    });
  })
  .catch((reason) => error("Error: " + reason));
</script>
