<template>
  <div class="flex h-full items-center justify-center">
    <h1 class="text-primary text-8xl font-bold">{{ projectName }}</h1>
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";

const projectName = ref("Unnamed Project");
</script>
