<template>
  <div class="border-surface h-16 max-h-16 min-h-16 border-b-4">
    <div class="flex h-full items-center gap-2 px-4 py-2">
      <div class="text-primary/80 text-xl">Currently Active:</div>
      <div v-auto-animate="{ duration: 75 }" class="flex gap-2">
        <div
          v-for="item in activeItems"
          :key="item"
          class="bg-primary text-primary-contrast rounded-md p-2 font-mono font-bold"
        >
          {{ item }}
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, inject } from "vue";
import { PatternState, TrackState } from "../App.vue";
import { get } from "@vueuse/core";

const { tracks } = inject<TrackState>("track-state")!;
const { patterns } = inject<PatternState>("pattern-state")!;

const activeItems = computed(() => {
  let patternsInner = get(patterns) ?? {};
  return Object.values(get(tracks) ?? {})
    .filter((track) => track.active)
    .map((track) => track.patterns)
    .flat()
    .map((patName) =>
      [
        patternsInner[patName].messages.map((msg) => msg.payload.path),
        patternsInner[patName].midi_path,
      ].flat(),
    )
    .flat()
    .filter((path) => path != "/" && path != "")
    .map((path) => path.split("/")[2])
    .map((item) => {
      switch (item) {
        case "hihat":
        case "kick":
        case "snare":
          return "drums";
        default:
          return item;
      }
    })
    .filter((path, index, self) => self.indexOf(path) === index)
    .sort();
});
</script>
