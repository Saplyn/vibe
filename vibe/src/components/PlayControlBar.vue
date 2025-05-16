<template>
  <div class="border-surface flex gap-2 border-b-4 px-4 py-2">
    <!-- LYN: Status -->
    <Button
      @click="toggleStatusPopover"
      :severity="statusButtonStyle.severity"
      variant="outlined"
    >
      <template #icon>
        <span class="material-symbols-rounded">
          {{ statusButtonStyle.icon }}
        </span>
      </template>
    </Button>

    <!-- LYN: Status Popover -->
    <Popover ref="statusPopover">
      <Message
        class="mt-1"
        :severity="connected ? 'success' : 'error'"
        variant="outlined"
      >
        <template #icon>
          <span class="material-symbols-rounded">dns</span>
        </template>
        {{ connected ? "Connected to" : "Disconnected from" }}
        <span class="font-mono">vibed</span> server
        <span v-if="connected">
          at
          <span class="font-mono underline">{{ wsAddr }}</span>
        </span>
      </Message>
    </Popover>

    <!-- LYN: BPM -->
    <span class="w-24">
      <FloatLabel variant="on">
        <InputNumber
          id="bpm"
          :disabled="bpm == null"
          v-model="bpm"
          showButtons
          fluid
          @update:model-value="
            send({
              action: 'TickerSetBpm',
              payload: { bpm: bpm! },
            })
          "
        />
        <label for="bpm">BPM</label>
      </FloatLabel>
    </span>

    <!-- LYN: Puase / Play -->
    <Button
      :disabled="playing == null"
      @click="
        playing
          ? send({ action: 'TickerPause' })
          : send({ action: 'TickerPlay' })
      "
    >
      <template #icon>
        <span class="material-symbols-rounded">
          {{ playing ? "pause" : "play_arrow" }}
        </span>
      </template>
    </Button>

    <!-- LYN: Stop -->
    <Button :disabled="playing == null" @click="send({ action: 'TickerStop' })">
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
import { computed, inject, ref, watch } from "vue";
import { Vibed } from "../App.vue";
import { set } from "@vueuse/core";

// LYN: Vibed Communication
const { connected, wsAddr, cmd, send } = inject<Vibed>("vibed")!;
watch(cmd, (cmd) => {
  switch (cmd!.action) {
    case "TickerPlaying":
      set(playing, true);
      break;
    case "TickerPaused":
      set(playing, false);
      break;
    case "TickerStopped":
      set(playing, false);
      break;
    case "TickerBpmUpdated":
      set(bpm, cmd.payload.bpm);
      break;
    case "ResponseTickerBpm":
      set(bpm, cmd.payload.bpm);
      break;
    case "ResponseTickerPlaying":
      set(playing, cmd.payload.playing);
      break;
  }
});
watch(connected, async (connected) => {
  if (connected) {
    send({ action: "RequestTickerBpm" });
    send({ action: "RequestTickerPlaying" });
  } else {
    bpm.value = undefined;
    playing.value = undefined;
  }
});

const bpm = ref<number>();
const playing = ref<boolean>();

// LYN: Status Popover
const statusPopover = ref();
function toggleStatusPopover(event: MouseEvent) {
  statusPopover.value.toggle(event);
}

const statusButtonStyle = computed(() => {
  if (connected.value) {
    return { icon: "wifi_tethering", severity: "success" };
    return { icon: "wifi_tethering_error", severity: "warn" };
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
