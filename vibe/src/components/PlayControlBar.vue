<template>
  <div class="border-surface flex h-[62px] gap-2 border-b-4 px-4 py-2">
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
      <div class="flex flex-col gap-1">
        <!-- LYN: Vibed Status -->
        <Message
          class="mt-1"
          :severity="connected ? 'success' : 'error'"
          variant="outlined"
        >
          <template #icon>
            <span class="material-symbols-rounded">dns</span>
          </template>
          {{ connected ? "Connected to" : "Disconnected from" }}
          <span class="font-mono">vibed</span> server at
          <span class="font-mono underline">{{ wsAddr }}</span>
        </Message>

        <!-- LYN: Target Connection -->
        <Message
          class="mt-1"
          :severity="established ? 'success' : 'error'"
          variant="outlined"
        >
          <template #icon>
            <span class="material-symbols-rounded">radio</span>
          </template>
          <span v-if="connected">
            <span class="font-mono">vibed</span> host
            {{
              established
                ? "connected to TCP server at"
                : "disconnected from TCP server at"
            }}
            <span class="font-mono underline">{{ commAddr }}</span>
          </span>
          <span v-else>
            <span class="font-mono">vibed</span> host TCP connection status
            unavailable
          </span>
        </Message>
      </div>
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
      option-disabled="disabled.value"
      :disabled="!connected"
    >
      <template #option="slotProps">
        <span class="material-symbols-rounded">
          {{ slotProps.option.icon }}
        </span>
        {{ slotProps.option.label }}
      </template>
    </SelectButton>

    <ProgressBar
      :value="((tick + 1) / 16) * 100"
      class="h-full grow"
      :pt:value:class="
        'duration-[0ms] ' +
        (tick % 4 === 0 ? 'bg-primary-400 dark:bg-primary-300' : 'bg-primary')
      "
      :class="
        tick % 4 === 0 ? 'bg-primary-100 dark:bg-surface-600' : 'bg-surface'
      "
      pt:label:class="hidden"
    />
  </div>
</template>

<script setup lang="ts">
import { computed, inject, ref, watch } from "vue";
import { CommInfo, PatternEditing, Vibed } from "../App.vue";
import { get, set } from "@vueuse/core";

// LYN: Styling States
const bpm = ref<number>();
const playing = ref<boolean>();
const tick = ref<number>(-1);

// LYN: Status Popover
const statusPopover = ref();
const toggleStatusPopover = (e: MouseEvent) => get(statusPopover).toggle(e);
const statusButtonStyle = computed(() => {
  if (get(connected)) {
    if (get(established)) {
      return { icon: "wifi_tethering", severity: "success" };
    }
    return { icon: "wifi_tethering_error", severity: "warn" };
  }
  return { icon: "wifi_tethering_off", severity: "danger" };
});

// LYN: Context
const { name } = inject<PatternEditing>("pattern-editing")!;
const currContext = ref<{ value: string }>({ value: "track" });
const contexts = [
  {
    value: "track",
    icon: "queue_music",
    label: undefined,
    disabled: computed(() => false),
  },
  {
    value: "pattern",
    icon: "library_music",
    label: name,
    disabled: computed(() => get(name) == undefined),
  },
];
watch(name, (name) => {
  if (name == undefined) {
    set(currContext, { value: "track" });
  }
});

// LYN: Target Comm
const { addr: commAddr, established } = inject<CommInfo>("comm-info")!;

// LYN: Data Fetching & Update
const { connected, wsAddr, cmd, send, watchableResp } = inject<Vibed>("vibed")!;
watch([cmd, watchableResp], ([cmd, _]) => {
  switch (cmd!.action) {
    case "TickerPlaying":
      set(playing, true);
      break;
    case "TickerPaused":
      set(playing, false);
      break;
    case "TickerStopped":
      set(tick, -1);
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
    case "ResponseTickerTick":
      set(tick, cmd.payload.tick);
      break;
    case "TickerTick":
      set(tick, cmd.payload.tick);
      break;
  }
});
watch(connected, async (connected) => {
  if (connected) {
    send({ action: "RequestTickerBpm" });
    send({ action: "RequestTickerPlaying" });
    send({ action: "RequestTickerTick" });
  } else {
    set(bpm, undefined);
    set(playing, undefined);
    set(tick, -1);
  }
});
</script>
