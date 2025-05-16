<template>
  <div class="flex h-full flex-col items-center justify-center">
    <h1
      class="text-primary truncate text-2xl font-bold md:text-5xl lg:text-8xl"
    >
      {{ projectName }}
    </h1>

    <InputText v-model="vibedAddr" />
    <div>{{ wsAddr }}</div>
  </div>
</template>

<script setup lang="ts">
import { inject, ref, watch } from "vue";
import { Vibed } from "../App.vue";
import { get } from "@vueuse/core";

const projectName = ref("Unnamed Project");

// LYN: Vibed Addr
const { addr, wsAddr, changeAddr } = inject<Vibed>("vibed")!;
const vibedAddr = ref(get(addr));
watch(vibedAddr, (newAddr) => changeAddr(newAddr));
</script>
