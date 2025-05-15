<template>
  <div class="border-surface flex gap-2 border-b-4 px-4 py-2">
    <!-- LYN: Status -->
    <Button
      @click="toggleStatusPopover"
      :severity="statusButtonStyle.severity"
      variant="outlined"
    >
      <template #icon>
        <span class="material-symbols-rounded">{{
          statusButtonStyle.icon
        }}</span>
      </template>
    </Button>

    <!-- LYN: Status Popover -->
    <Popover ref="statusPopover">
      <Message
        class="mt-1"
        :severity="vibedConnected ? 'success' : 'error'"
        variant="outlined"
      >
        <template #icon>
          <span class="material-symbols-rounded">dns</span>
        </template>
        {{ vibedConnected ? "Connected" : "Disconnected" }}
        to <span class="font-mono">vibed</span> server
      </Message>
    </Popover>

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
import { computed, inject, ref } from "vue";
import { UseVibedReturn } from "../composables/useVibed";

const bpm = ref(120);
const playing = ref(false);

const { connected: vibedConnected, command } = inject(
  "vibed",
) as UseVibedReturn;

function play() {
  playing.value = true;
  command({ action: "TickerPlay" });
}
function pause() {
  playing.value = false;
  command({ action: "TickerPause" });
}
function stop() {
  playing.value = false;
  command({ action: "TickerStop" });
}
function setBpm() {
  command({
    action: "TickerSetBpm",
    payload: { bpm: bpm.value },
  });
}

// LYN: Status Popover
const statusPopover = ref();
function toggleStatusPopover(event: MouseEvent) {
  statusPopover.value.toggle(event);
}

const statusButtonStyle = computed(() => {
  if (vibedConnected.value) {
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
