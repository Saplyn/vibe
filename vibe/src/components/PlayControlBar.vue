<template>
  <div class="border-surface flex gap-2 border-b-4 px-4 py-2">
    <!-- LYN: Status -->
    <Button
      @click="statusDrawerVisible = true"
      :severity="statusButtonStyle.severity"
      variant="outlined"
    >
      <template #icon>
        <span class="material-symbols-rounded">{{
          statusButtonStyle.icon
        }}</span>
      </template>
    </Button>

    <!-- LYN: Status Drawer -->
    <Drawer
      v-model:visible="statusDrawerVisible"
      header="Vibe Info"
      position="bottom"
      class="h-auto"
    >
      <Message
        class="mt-1"
        :severity="vibedCommStatus === 'OPEN' ? 'success' : 'error'"
        variant="outlined"
      >
        <span class="font-mono">vibed</span> server
        {{ vibedCommStatus === "OPEN" ? "connected" : "disconnected" }}
      </Message>
    </Drawer>

    <!-- LYN: BPM -->
    <span class="w-24">
      <FloatLabel variant="on">
        <InputNumber
          id="bpm"
          v-model="bpm"
          showButtons
          fluid
          @update:model-value="setBpm()"
        />
        <label for="bpm">BPM</label>
      </FloatLabel>
    </span>

    <!-- LYN: Puase / Play -->
    <Button @click="playing ? pause() : play()">
      <template #icon>
        <span class="material-symbols-rounded">
          {{ playing ? "pause" : "play_arrow" }}
        </span>
      </template>
    </Button>

    <!-- LYN: Stop -->
    <Button @click="stop">
      <template #icon>
        <span class="material-symbols-rounded">stop</span>
      </template>
    </Button>

    <Divider layout="vertical" />

    <!-- LYN: Context Selector -->
    <SelectButton
      :allowEmpty="false"
      v-model="currContext"
      :options="contexts"
      option-label="value"
      data-key="value"
    >
      <template #option="slotProps">
        <span class="material-symbols-rounded">
          {{ slotProps.option.icon }}
        </span>
      </template>
    </SelectButton>

    <ProgressBar :value="50" class="h-full grow" />
  </div>
</template>

<script setup lang="ts">
import { UseWebSocketReturn } from "@vueuse/core";
import { computed, inject, ref } from "vue";
import { ServerCommand } from "../types";

const bpm = ref(120);
const playing = ref(false);

const { status: vibedCommStatus, send } = inject(
  "vibed-comm-ws",
) as UseWebSocketReturn<any>;

function play() {
  playing.value = true;
  let cmd: ServerCommand = {
    action: "TickerPlay",
  };
  send(JSON.stringify(cmd));
}
function pause() {
  playing.value = false;
  let cmd: ServerCommand = {
    action: "TickerPause",
  };
  send(JSON.stringify(cmd));
}
function stop() {
  playing.value = false;
  let cmd: ServerCommand = {
    action: "TickerStop",
  };
  send(JSON.stringify(cmd));
}
function setBpm() {
  console.log("uwu");
  let cmd: ServerCommand = {
    action: "TickerSetBpm",
    payload: { bpm: bpm.value },
  };
  send(JSON.stringify(cmd));
}

// LYN: Status Drawer
const statusDrawerVisible = ref(false);

const statusButtonStyle = computed(() => {
  if (vibedCommStatus.value === "OPEN") {
    return { icon: "wifi_tethering", severity: "success" };
    return { icon: "wifi_tethering_error", severity: "success" };
  } else {
    return { icon: "wifi_tethering_off", severity: "danger" };
  }
});

// LYN: Context
const currContext = ref<{ value: string; icon?: string }>({ value: "track" });
const contexts = [
  { value: "track", icon: "queue_music" },
  { value: "pattern", icon: "library_music" },
];
</script>
