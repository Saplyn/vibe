<template>
  <div class="h-full">
    <BlockUI class="flex h-full w-full flex-col" :blocked="!connected">
      <div
        class="border-surface dark:bg-surface-800 bg-surface-100 sticky top-0 z-50 flex min-h-[62px] items-center gap-2 border-b-4 border-dotted px-4 py-2"
      >
        <!-- LYN: Add New Track -->
        <Button
          label="Add New Track"
          @click="addTrackWrapper()"
          :disabled="!connected || trackNameToAdd == ''"
          :ref="addTrackButtonRef"
        >
          <template #icon>
            <span class="material-symbols-rounded">playlist_add</span>
          </template>
        </Button>

        <FloatLabel class="grow" variant="on">
          <InputText fluid :disabled="!connected" v-model="trackNameToAdd" />
          <label>New track name</label>
        </FloatLabel>
      </div>

      <!-- LYN: Tracks View -->
      <div class="flex flex-col gap-4 p-4">
        <div v-for="track in tracks" class="flex gap-2">
          <div class="flex h-20 w-80 min-w-50">
            <ButtonGroup
              class="outline-surface-300 dark:outline-surface-600 grow rounded-lg outline-2"
              :style="
                track.color != null || track.color != ''
                  ? `outline-color: ${track.color} !important;`
                  : ''
              "
            >
              <!-- LYN: Make Active -->
              <Button
                class="border-surface grow basis-3/4 justify-start border-1"
                @click="
                  activeStateMatch(track.name)
                    ? makeTrackActiveWrapper(track.name, !track.active)
                    : makeTrackActiveWrapper(track.name, track.active, true)
                "
                :label="track.name"
                :severity="
                  track.active
                    ? track.progress == null
                      ? 'contrast'
                      : 'primary'
                    : track.progress != null
                      ? 'contrast'
                      : 'secondary'
                "
              >
                <template #icon>
                  <span class="material-symbols-rounded">
                    {{ track.active ? "music_note" : "music_off" }}
                  </span>
                </template>
              </Button>

              <!-- LYN: Loop -->
              <Button
                class="border-surface grow basis-1/4 border-1"
                @click="makeTrackLoopWrapper(track.name, !track.loop)"
                :severity="track.loop ? 'primary' : 'secondary'"
              >
                <template #icon>
                  <span class="material-symbols-rounded">replay</span>
                </template>
              </Button>

              <!-- LYN: Edit -->
              <Button
                class="border-surface grow basis-1/6 border-1"
                @click="preparEditTrack(track.name)"
                severity="secondary"
              >
                <template #icon>
                  <span class="material-symbols-rounded">edit</span>
                </template>
              </Button>
            </ButtonGroup>
          </div>

          <!-- LYN: Pattern Display -->
          <div class="flex h-full grow flex-col gap-2">
            <div class="flex grow gap-2">
              <div
                v-if="track.patterns.length === 0"
                class="dark:bg-surface-700 bg-surface-200 text-surface-400 dark:text-surface-500 flex grow items-center justify-center rounded-md font-mono text-lg"
              >
                <span class="material-symbols-rounded">block</span>
              </div>
              <div
                v-else
                v-for="pat in track.patterns"
                class="dark:bg-surface-700 bg-surface-200 text-surface-500 dark:text-surface-400 flex grow items-center justify-center rounded-md font-mono text-lg"
                :style="{
                  flexBasis: `${Math.min(patternIsValidAndLength(pat) ?? 1, 100)}%`,
                }"
              >
                {{ pat }}({{ patternIsValidAndLength(pat) ?? "?" }})
              </div>
            </div>

            <ProgressBar
              :value="getTrackProgressPercent(track.name)"
              :pt:value:class="
                trackIsEnding(track.name)
                  ? 'duration-[0ms] bg-red-500/60'
                  : trackIsStarting(track.name)
                    ? 'duration-[0ms] bg-green-500/60'
                    : 'duration-[0ms] bg-primary/60'
              "
              pt:label:class="hidden"
            />
          </div>
        </div>
      </div>
    </BlockUI>

    <!-- LYN: Edit Track -->
    <Dialog v-model:visible="editTrackDialogVisible" header="Edit Track" modal>
      <div class="flex flex-col gap-2">
        <div class="flex items-center justify-center font-mono text-lg">
          {{ editingName }}
        </div>

        <div class="flex gap-2" v-for="(_, i) in trackEditing!.patterns.length">
          <!-- LYN: Delete Pattern -->
          <Button severity="danger" variant="text" @click="removePatAt(i)">
            <template #icon>
              <span class="material-symbols-rounded">close</span>
            </template>
          </Button>

          <!-- LYN: Pattern Name -->
          <InputGroup>
            <Button
              severity="secondary"
              variant="outlined"
              @click="movePatUp(i)"
              :disabled="i == 0"
            >
              <template #icon>
                <span class="material-symbols-rounded">
                  keyboard_double_arrow_up
                </span>
              </template>
            </Button>

            <AutoComplete
              v-model="trackEditing!.patterns[i]"
              :suggestions="filteredPatternNames"
              @complete="search"
            />

            <Button
              severity="secondary"
              variant="outlined"
              @click="movePatDown(i)"
              :disabled="i == trackEditing!.patterns.length - 1"
            >
              <template #icon>
                <span class="material-symbols-rounded">
                  keyboard_double_arrow_down
                </span>
              </template>
            </Button>
          </InputGroup>

          <!-- LYN: Pattern Length -->
          <div
            class="flex w-10 min-w-10 items-center justify-center font-mono text-xl"
          >
            <div
              v-if="
                patternIsValidAndLength(trackEditing!.patterns[i]) != undefined
              "
            >
              {{ patternIsValidAndLength(trackEditing!.patterns[i]) }}
            </div>
            <div v-else class="text-surface-400 dark:text-surface-500">?</div>
          </div>
        </div>

        <FloatLabel class="grow" variant="on">
          <InputText v-model="trackEditing!.color" fluid />
          <label>Color</label>
        </FloatLabel>

        <!-- LYN: Make Edit -->
        <div class="mt-2 flex grow gap-2">
          <Button
            class="grow"
            @click="confirmDelPattern($event)"
            severity="danger"
          >
            <template #icon>
              <span class="material-symbols-rounded">delete</span>
            </template>
          </Button>
          <Button class="grow" label="Add Pattern" @click="editAddPattern()">
            <template #icon>
              <span class="material-symbols-rounded">add</span>
            </template>
          </Button>

          <Button
            class="grow"
            label="Make Edit"
            @click="makeEdit()"
            :variant="dirty ? '' : 'outlined'"
            :disabled="!dirty"
          >
            <template #icon>
              <span class="material-symbols-rounded">edit_square</span>
            </template>
          </Button>
        </div>
      </div>
    </Dialog>
  </div>
</template>

<script setup lang="ts">
import { computed, inject, ref, watch } from "vue";
import { PatternState, PlayingState, TrackState, Vibed } from "../App.vue";
import { get, onKeyStroke, set, useFocus } from "@vueuse/core";
import { Track } from "../types/models";
import { cloneDeep, isEqual } from "lodash";
import { useConfirm } from "primevue";

const { connected } = inject<Vibed>("vibed")!;
const { patterns } = inject<PatternState>("pattern-state")!;
const { playing } = inject<PlayingState>("playing")!;
const {
  tracks,
  delTrack,
  addTrack,
  editTrack,
  makeTrackActive,
  makeTrackLoop,
} = inject<TrackState>("track-state")!;

// LYN: Auto Complete
const allPatternNames = computed(() => {
  let pats = get(patterns);
  return pats == undefined
    ? []
    : Object.values(pats).map((item: { name: string }) => item.name);
});
const filteredPatternNames = ref<string[]>([]);
function search(event: any) {
  if (!event.query.trim().length) {
    set(filteredPatternNames, [...get(allPatternNames)]);
  } else {
    set(
      filteredPatternNames,
      get(allPatternNames).filter((name) => {
        return name.toLowerCase().startsWith(event.query.toLowerCase());
      }),
    );
  }
}

// LYN: Add Track
const trackNameToAdd = ref<string>("");
function addTrackWrapper() {
  addTrack(get(trackNameToAdd));
  set(trackNameToAdd, "");
}
const addTrackButtonRef = ref();
const { focused: addTrackButtonFocused } = useFocus(addTrackButtonRef);
onKeyStroke(
  "Enter",
  (e) => {
    if (addTrackButtonFocused && connected && get(trackNameToAdd) != "") {
      addTrackWrapper();
      e.preventDefault();
    }
  },
  { dedupe: true },
);

// LYN: Delete Track
const confirm = useConfirm();
function confirmDelPattern(event: MouseEvent) {
  confirm.require({
    target: event.currentTarget as any,
    message: "Confirm deletion?",
    icon: "pi pi-info-circle",
    rejectProps: {
      label: "Cancel",
      severity: "secondary",
      outlined: true,
    },
    acceptProps: {
      label: "Delete",
      severity: "danger",
    },
    accept: () => {
      let name = get(editingName);
      if (name != undefined) {
        set(editTrackDialogVisible, false);
        delTrack(name);
      }
    },
  });
}

// LYN: Edit Track
const editingName = ref<string>();
const editTrackDialogVisible = ref(false);
function preparEditTrack(name: string) {
  set(editTrackDialogVisible, true);
  set(editingName, name);
}
const trackOriginal = ref<Track>();
const trackEditing = ref<Track>();
watch(
  editingName,
  (name) => {
    if (name != undefined) {
      let track = get(tracks)?.[name];
      if (track != undefined) {
        set(trackOriginal, cloneDeep(track));
        set(trackEditing, cloneDeep(track));
      } else {
        set(trackEditing, undefined);
      }
    } else {
      set(trackEditing, undefined);
    }
  },
  { immediate: true },
);

// LYN: Make Edit
function makeEdit() {
  if (get(editingName) != undefined) {
    editTrack(get(editingName)!, get(trackEditing)!);
  }
  set(editTrackDialogVisible, false);
  set(editingName, undefined);
}
const dirty = computed(() => {
  return !isEqual(get(trackOriginal), get(trackEditing));
});

// LYN: Edit Pattern In Track
function editAddPattern() {
  let track = get(trackEditing);
  if (track != undefined) {
    track.patterns.push("");
  }
}
function removePatAt(i: number) {
  let track = get(trackEditing);
  if (track != undefined) {
    track.patterns.splice(i, 1);
  }
}
function movePatUp(i: number) {
  let track = get(trackEditing);
  if (track != undefined) {
    let pat = track.patterns[i];
    track.patterns.splice(i, 1);
    track.patterns.splice(i - 1, 0, pat);
  }
}
function movePatDown(i: number) {
  let track = get(trackEditing);
  if (track != undefined) {
    let pat = track.patterns[i];
    track.patterns.splice(i, 1);
    track.patterns.splice(i + 1, 0, pat);
  }
}

// LYN: Track Loop
function makeTrackLoopWrapper(name: string, loop: boolean) {
  let track = get(tracks)?.[name];
  if (track != undefined) {
    makeTrackLoop(name, loop);
  }
}

// LYN: Track Play
function activeStateMatch(name: string): boolean {
  let track = get(tracks)?.[name];
  if (track == undefined) {
    return true;
  }
  return (
    !get(playing) ||
    (track.active && track.progress != null) ||
    (!track.active && track.progress == null)
  );
}
function makeTrackActiveWrapper(
  name: string,
  active: boolean,
  force: boolean = false,
) {
  let track = get(tracks)?.[name];
  if (track != undefined) {
    makeTrackActive(name, active, force);
  }
}

// LYN: Check Pattern Valid & Length
function patternIsValidAndLength(name: string): number | undefined {
  if (name == "") {
    return undefined;
  }
  return get(patterns)?.[name]?.page_count;
}

// LYN: Track Length
function trackTickCount(name: string): number | undefined {
  if (name == "") {
    return undefined;
  }
  let track = get(tracks)?.[name];
  if (track == undefined) {
    return undefined;
  }
  let length = 0;
  for (let i = 0; i < track.patterns.length; i++) {
    let pat = get(patterns)?.[track.patterns[i]];
    if (pat != undefined) {
      length += pat.page_count;
    }
  }
  return length * 4;
}

// LYN: Track Progress Styling
function getTrackProgressPercent(name: string): number {
  let track = get(tracks)?.[name];
  if (track == undefined) {
    return 0;
  }
  let progress = track.progress;
  if (progress == undefined) {
    return 0;
  }
  if (progress == 0) {
    return 100;
  }
  return (progress / (trackTickCount(name) ?? 0)) * 100;
}
function trackIsEnding(name: string): boolean {
  let track = get(tracks)?.[name];
  if (track == undefined) {
    return false;
  }
  let progress = track.progress;
  if (progress == undefined) {
    return false;
  }
  if (progress == 0) {
    return false;
  }
  return progress > (trackTickCount(name) ?? 0) - 16;
}
function trackIsStarting(name: string): boolean {
  let track = get(tracks)?.[name];
  if (track == undefined) {
    return false;
  }
  let progress = track.progress;
  if (progress == undefined) {
    return false;
  }
  if (progress == 0) {
    return false;
  }
  return progress < 16;
}
</script>
