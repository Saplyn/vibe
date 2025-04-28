<template>
  <div>
    {{ status }}
    <Button @click="play" icon="pi pi-play" />
    <Button @click="pause" icon="pi pi-pause" />
    <Button @click="stop" icon="pi pi-stop" />
  </div>
</template>

<script setup lang="ts">
import { useWebSocket } from "@vueuse/core";

const { status, send } = useWebSocket("ws://127.0.0.1:8000", {
  autoReconnect: true,
});

function play() {
  send('"TickerPlay"');
}
function pause() {
  send('"TickerPause"');
}
function stop() {
  send('"TickerStop"');
}
</script>
