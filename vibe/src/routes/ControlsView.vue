<template>
  <BlockUI class="flex h-full w-full" :blocked="!connected">
    <Splitter
      class="grow rounded-none border-none bg-transparent"
      layout="vertical"
      :pt:gutterHandle:class="'outline-1 outline-surface-500 bg-surface-500'"
    >
      <SplitterPanel :min-size="30">events</SplitterPanel>

      <SplitterPanel :min-size="40">sliders</SplitterPanel>
    </Splitter>
  </BlockUI>
</template>

<script setup lang="ts">
import { inject } from "vue";
import { Vibed } from "../App.vue";

const { connected } = inject<Vibed>("vibed")!;
</script>
