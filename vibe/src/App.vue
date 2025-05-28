<template>
  <div class="flex h-screen w-screen flex-col justify-between">
    <PlayControlBar />

    <main class="grow overflow-auto">
      <RouterView />
    </main>

    <nav
      class="border-surface grid grid-flow-col gap-2 rounded-t-lg border-t-4 px-2 pt-2"
      :class="isFullscreen ? '' : 'pb-2'"
    >
      <Button
        v-for="route in $router.getRoutes()"
        :severity="$route.path === route.path ? 'primary' : 'secondary'"
        :class="isFullscreen ? 'rounded-b-none' : ''"
        :pt:label:class="
          $route.path === route.path
            ? 'font-bold'
            : 'font-bold hidden md:inline'
        "
        :label="route.name?.toString()"
        @click="$router.push(route.path)"
      >
        <template #icon>
          <span class="material-symbols-rounded">{{ route.meta.icon }}</span>
        </template>
      </Button>
    </nav>

    <Toast />
    <ConfirmPopup />
  </div>
</template>

<script setup lang="ts">
import { getCurrentWindow } from "@tauri-apps/api/window";
import {
  computed,
  ComputedRef,
  DeepReadonly,
  onMounted,
  provide,
  readonly,
  Ref,
  ref,
  UnwrapNestedRefs,
  watch,
} from "vue";
import { get, set, useWebSocket } from "@vueuse/core";
import { ClientCommand, ServerCommand } from "./types/command";
import { Pattern, Track, Event, Slider } from "./types/models";
import { useToast } from "primevue/usetoast";
import { info } from "@tauri-apps/plugin-log";

// LYN: Vibed
const addr = ref("localhost:8000");
const changeAddr = (newAddr: string) => set(addr, newAddr);
const watchableResp = ref(false);
const wsAddr = computed(() => `ws://${get(addr)}`);
const ws = useWebSocket(wsAddr, {
  autoReconnect: true,
  onMessage() {
    info(`${get(cmd).action}`);
    set(watchableResp, !get(watchableResp));
  },
});
const connected = computed(() => ws.status.value === "OPEN");
const cmd = computed(() => JSON.parse(ws.data.value) as ClientCommand);
const send = (cmd: ServerCommand) => ws.send(JSON.stringify(cmd));

// LYN: Vibed (provide)
export type Vibed = {
  addr: DeepReadonly<UnwrapNestedRefs<Ref<string, string>>>;
  changeAddr: (newAddr: string) => void;
  wsAddr: ComputedRef<string>;
  connected: ComputedRef<boolean>;
  cmd: ComputedRef<ClientCommand>;
  send: (cmd: ServerCommand) => void;
  watchableResp: Ref<boolean, boolean>;
};
provide<Vibed>("vibed", {
  addr: readonly(addr),
  changeAddr,
  wsAddr,
  connected,
  cmd,
  send,
  watchableResp,
});

// LYN: Toast
const toast = useToast();
watch([cmd, watchableResp], ([cmd, _]) => {
  if (cmd.action === "Notify") {
    info(cmd.payload.severity);
    toast.add({
      severity: cmd.payload.severity,
      summary: cmd.payload.summary,
      detail: cmd.payload.detail,
      life: 3000,
    });
  }
});

// LYN: Fullscreen Detection
const isFullscreen = ref(false);
onMounted(async () => {
  isFullscreen.value = await getCurrentWindow().isMaximized();

  getCurrentWindow().listen("tauri://resize", async (_) => {
    isFullscreen.value = await getCurrentWindow().isMaximized();
  });
});

// LYN: Patterns
const patterns = ref<Record<string, Pattern>>();
const sortedPatterns = computed(() => {
  const entries = Object.entries(get(patterns) ?? {});
  entries.sort(([a], [b]) => a.localeCompare(b));
  return Object.fromEntries(entries) as Record<string, Pattern>;
});
const addPattern = (name: string) =>
  send({ action: "PatternAdd", payload: { name } });
const delPattern = (name: string) =>
  send({ action: "PatternDelete", payload: { name } });
const editPattern = (name: string, pattern: Pattern) =>
  send({ action: "PatternEdit", payload: { name, pattern } });
export type PatternState = {
  patterns: DeepReadonly<
    UnwrapNestedRefs<
      Ref<
        Record<string, Pattern> | undefined,
        Record<string, Pattern> | undefined
      >
    >
  >;
  addPattern: (name: string) => void;
  delPattern: (name: string) => void;
  editPattern: (name: string, pattern: Pattern) => void;
};
provide<PatternState>("pattern-state", {
  patterns: readonly(sortedPatterns),
  addPattern,
  delPattern,
  editPattern,
});
watch([cmd, watchableResp], ([cmd, _]) => {
  switch (cmd!.action) {
    case "PatternAdded":
      set(patterns, {
        ...get(patterns),
        [cmd.payload.name]: cmd.payload.pattern,
      });
      break;
    case "PatternDeleted":
      const newPatterns = { ...get(patterns) };
      delete newPatterns[cmd.payload.name];
      if (get(patternName) === cmd.payload.name) {
        set(patternName, undefined);
      }
      set(patterns, newPatterns);
      break;
    case "PatternEdited":
      set(patterns, {
        ...get(patterns),
        [cmd.payload.name]: cmd.payload.pattern,
      });
      break;
  }
});

// LYN: Tracks
const tracks = ref<Record<string, Track>>();
const sortedTracks = computed(() => {
  const entries = Object.entries(get(tracks) ?? {});
  entries.sort(([a], [b]) => a.localeCompare(b));
  return Object.fromEntries(entries) as Record<string, Track>;
});
const addTrack = (name: string) =>
  send({ action: "TrackAdd", payload: { name } });
const delTrack = (name: string) =>
  send({ action: "TrackDelete", payload: { name } });
const editTrack = (name: string, track: Track) =>
  send({ action: "TrackEdit", payload: { name, track } });
const makeTrackActive = (name: string, active: boolean, force: boolean) =>
  send({ action: "TrackMakeActive", payload: { name, active, force } });
const makeTrackLoop = (name: string, loop: boolean) =>
  send({ action: "TrackMakeLoop", payload: { name, loop } });
export type TrackState = {
  tracks: DeepReadonly<
    UnwrapNestedRefs<
      Ref<Record<string, Track> | undefined, Record<string, Track> | undefined>
    >
  >;
  addTrack: (name: string) => void;
  delTrack: (name: string) => void;
  editTrack: (name: string, track: Track) => void;
  makeTrackActive: (name: string, active: boolean, force: boolean) => void;
  makeTrackLoop: (name: string, loop: boolean) => void;
};
provide<TrackState>("track-state", {
  tracks: readonly(sortedTracks),
  addTrack,
  delTrack,
  editTrack,
  makeTrackActive,
  makeTrackLoop,
});
watch([cmd, watchableResp], ([cmd, _]) => {
  switch (cmd!.action) {
    case "TrackAdded":
      set(tracks, {
        ...get(tracks),
        [cmd.payload.name]: cmd.payload.track,
      });
      break;
    case "TrackDeleted":
      const newTracks = { ...get(tracks) };
      delete newTracks[cmd.payload.name];
      set(tracks, newTracks);
      break;
    case "TrackEdited":
      set(tracks, {
        ...get(tracks),
        [cmd.payload.name]: cmd.payload.track,
      });
      break;
    case "TrackProgressUpdate":
      const newTracksProgress = { ...get(tracks) };
      newTracksProgress[cmd.payload.name].progress = cmd.payload.progress;
      set(tracks, newTracksProgress);
      break;
    case "TrackMadeActive":
      const newTracksActive = { ...get(tracks) };
      newTracksActive[cmd.payload.name].active = cmd.payload.active;
      set(tracks, newTracksActive);
      break;
    case "TrackMadeLoop":
      const newTracksLoop = { ...get(tracks) };
      newTracksLoop[cmd.payload.name].loop = cmd.payload.loop;
      set(tracks, newTracksLoop);
      break;
  }
});

// LYN: Events

const events = ref<Record<string, Event>>();
const addEvent = (name: string) =>
  send({ action: "EventAdd", payload: { name } });
const delEvent = (name: string) =>
  send({ action: "EventDelete", payload: { name } });
const editEvent = (name: string, event: Event) =>
  send({ action: "EventEdit", payload: { name, event } });
const fireEvent = (name: string) =>
  send({ action: "EventFire", payload: { name } });
export type EventState = {
  events: DeepReadonly<
    UnwrapNestedRefs<Ref<Record<string, Event> | undefined>>
  >;
  addEvent: (name: string) => void;
  delEvent: (name: string) => void;
  editEvent: (name: string, event: Event) => void;
  fireEvent: (name: string) => void;
};
provide<EventState>("event-state", {
  events: readonly(events),
  addEvent,
  delEvent,
  editEvent,
  fireEvent,
});
watch([cmd, watchableResp], ([cmd, _]) => {
  switch (cmd!.action) {
    case "EventAdded":
      set(events, {
        ...get(events),
        [cmd.payload.name]: cmd.payload.event,
      });
      break;
    case "EventDeleted":
      const newEvents = { ...get(events) };
      delete newEvents[cmd.payload.name];
      set(events, newEvents);
      break;
    case "EventEdited":
      set(events, {
        ...get(events),
        [cmd.payload.name]: cmd.payload.event,
      });
      break;
  }
});

// LYN: Sliders

const sliders = ref<Record<string, Slider>>();
const addSlider = (name: string) =>
  send({ action: "SliderAdd", payload: { name } });
const delSlider = (name: string) =>
  send({ action: "SliderDelete", payload: { name } });
const editSlider = (name: string, slider: Slider) =>
  send({ action: "SliderEdit", payload: { name, slider } });
const setSliderVal = (name: string, val: number) =>
  send({ action: "SliderSetVal", payload: { name, val } });
export type SliderState = {
  sliders: DeepReadonly<
    UnwrapNestedRefs<Ref<Record<string, Slider> | undefined>>
  >;
  addSlider: (name: string) => void;
  delSlider: (name: string) => void;
  editSlider: (name: string, slider: Slider) => void;
  setSliderVal: (name: string, val: number) => void;
};
provide<SliderState>("slider-state", {
  sliders: readonly(sliders),
  addSlider,
  delSlider,
  editSlider,
  setSliderVal,
});
watch([cmd, watchableResp], ([cmd, _]) => {
  switch (cmd!.action) {
    case "SliderAdded":
      set(sliders, {
        ...get(sliders),
        [cmd.payload.name]: cmd.payload.slider,
      });
      break;
    case "SliderDeleted":
      const newSliders = { ...get(sliders) };
      delete newSliders[cmd.payload.name];
      set(sliders, newSliders);
      break;
    case "SliderEdited":
      set(sliders, {
        ...get(sliders),
        [cmd.payload.name]: cmd.payload.slider,
      });
      break;
    case "SliderValSet":
      const newSlidersVal = { ...get(sliders) };
      newSlidersVal[cmd.payload.name].val = cmd.payload.val;
      set(sliders, newSlidersVal);
      break;
  }
});

// LYN: Pattern Editing
const patternName = ref<string>();
const changeEditing = (name: string | null) =>
  set(patternName, name ?? undefined);
watch(patterns, (patterns) => {
  let name = get(patternName);
  if (name != undefined && patterns?.[name] == undefined) {
    set(patternName, undefined);
  }
});

// LYN: Pattern Editing (provide)
export type PatternEditing = {
  name: DeepReadonly<
    UnwrapNestedRefs<Ref<string | undefined, string | undefined>>
  >;
  change: (name: string | null) => void;
};
provide<PatternEditing>("pattern-editing", {
  name: readonly(patternName),
  change: changeEditing,
});

// LYN: Project Info
const projectName = ref<string>();
const changeProjectName = (name: string) => {
  send({
    action: "SetProjectName",
    payload: { name },
  });
  set(projectName, name);
};
export type ProjectInfo = {
  name: DeepReadonly<
    UnwrapNestedRefs<Ref<string | undefined, string | undefined>>
  >;
  change: (name: string) => void;
};
provide<ProjectInfo>("project-info", {
  name: readonly(projectName),
  change: changeProjectName,
});
watch([cmd, watchableResp], ([cmd, _]) => {
  switch (cmd!.action) {
    case "ProjectNameUpdated":
      set(projectName, cmd.payload.name);
      break;
  }
});

// LYN: Target Address
const commAddr = ref<string>();
const established = ref<boolean>();
const changeCommAddr = (newAddr: string) => {
  send({
    action: "CommChangeAddr",
    payload: { addr: newAddr },
  });
  set(commAddr, newAddr);
};
export type CommInfo = {
  addr: DeepReadonly<
    UnwrapNestedRefs<Ref<string | undefined, string | undefined>>
  >;
  established: DeepReadonly<
    UnwrapNestedRefs<Ref<boolean | undefined, boolean | undefined>>
  >;
  change: (name: string) => void;
};
provide<CommInfo>("comm-info", {
  addr: readonly(commAddr),
  established: readonly(established),
  change: changeCommAddr,
});

// LYN: Playing
const playing = ref<boolean>();
const setPlaying = (newPlaying?: boolean) => set(playing, newPlaying);
export type PlayingState = {
  playing: DeepReadonly<
    UnwrapNestedRefs<Ref<boolean | undefined, boolean | undefined>>
  >;
  setPlaying: (newPlaying: boolean) => void;
};
provide<PlayingState>("playing", {
  playing: readonly(playing),
  setPlaying,
});

// LYN: Data Fetching
watch([cmd, watchableResp], ([cmd, _]) => {
  switch (cmd!.action) {
    case "ResponseProjectName":
      set(projectName, cmd.payload.name);
      break;
    case "ResponseCommAddr":
      set(commAddr, cmd.payload.addr);
      break;
    case "ResponseCommStatus":
      set(established, cmd.payload.established);
      break;
    case "CommStatusChanged":
      set(established, cmd.payload.established);
      break;
    case "ResponseAllTracks":
      set(tracks, cmd.payload.tracks);
      break;
    case "ResponseAllPatterns":
      set(patterns, cmd.payload.patterns);
      break;
    case "ResponseAllEvents":
      set(events, cmd.payload.events);
      break;
    case "ResponseAllSliders":
      set(sliders, cmd.payload.sliders);
      break;
  }
});
watch(connected, async (connected) => {
  if (connected) {
    send({ action: "RequestProjectName" });
    send({ action: "RequestCommAddr" });
    send({ action: "RequestCommStatus" });
    send({ action: "RequestAllTracks" });
    send({ action: "RequestAllPatterns" });
    send({ action: "RequestAllEvents" });
    send({ action: "RequestAllSliders" });
  } else {
    set(projectName, undefined);
    set(commAddr, undefined);
    set(established, undefined);
    set(tracks, undefined);
    set(patterns, undefined);
    set(events, undefined);
    set(sliders, undefined);
  }
});
</script>
