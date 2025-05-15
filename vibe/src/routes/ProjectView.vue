<template>
  <div class="flex h-full items-center justify-center">
    <h1
      class="text-primary truncate text-2xl font-bold md:text-5xl lg:text-8xl"
    >
      {{ projectName }}
    </h1>
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";

const projectName = ref("Unnamed Project");
</script>
