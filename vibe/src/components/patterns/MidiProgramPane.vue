<template>
  <div class="flex h-full">
    <div v-for="_ in 16" class="flex grow flex-col">
      uwu
      <SelectButton
        v-model="value"
        :options="options"
        class="grow flex-col"
        pt:pcToggleButton:root:class="rounded-none font-mono grow flex items-center justify-center"
        :pt:pcToggleButton:content="
          (opt: any) => ({
            class: opt.context.active
              ? 'h-full bg-primary text-primary-contrast font-bold'
              : 'h-full',
          })
        "
      />
      uwu
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";

const value = ref();
const options = ref(Array.from({ length: 12 }, (_, i) => i + 1));
</script>
