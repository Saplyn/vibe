<template>
  <div class="h-full">
    <BlockUI class="flex h-full w-full" :blocked="!connected"> </BlockUI>
  </div>
</template>

<script setup lang="ts">
import { inject } from "vue";
import { TrackState, Vibed } from "../App.vue";

const { connected } = inject<Vibed>("vibed")!;

const { tracks } = inject<TrackState>("track-state")!;
</script>
