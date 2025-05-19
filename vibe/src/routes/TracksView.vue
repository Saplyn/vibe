<template>
  <div class="h-full">
    <BlockUI class="flex h-full w-full flex-col" :blocked="!connected">
      <div
        class="border-surface flex min-h-[62px] items-center gap-2 border-b-4 border-dotted px-4 py-2"
      >
        <!-- LYN: Add New Track -->
        <Button
          label="Add New Track"
          @click="addTrackWrapper()"
          :disabled="!connected || trackNameToAdd == ''"
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
      <div class="flex flex-col gap-2 p-2">
        <div v-for="track in tracks" class="flex">
          <div class="flex h-18 w-50 min-w-50">
            <ButtonGroup class="grow">
              <Button :label="track.name" fluid class="justify-start font-mono">
              </Button>

              <Button severity="danger">
                <template #icon>
                  <span class="material-symbols-rounded">delete</span>
                </template>
              </Button>
            </ButtonGroup>
          </div>

          {{ track }}
        </div>
      </div>

      <!-- LYN: Edit Track -->
      <Dialog v-model="editTrackDialogVisible"> </Dialog>
    </BlockUI>
  </div>
</template>

<script setup lang="ts">
import { inject, ref } from "vue";
import { PatternState, TrackState, Vibed } from "../App.vue";
import { get } from "@vueuse/core";

const { connected } = inject<Vibed>("vibed")!;

const { patterns } = inject<PatternState>("pattern-state")!;
const { tracks, delTrack, addTrack, editTrack } =
  inject<TrackState>("track-state")!;

// LYN: Add Track
const trackNameToAdd = ref<string>("");
function addTrackWrapper() {
  addTrack(get(trackNameToAdd));
}

// LYN: Edit Track
const editTrackDialogVisible = ref(false);
</script>
