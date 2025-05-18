<template>
  <div class="flex h-full flex-col items-center justify-center">
    <h1 class="text-primary m-8 text-2xl font-bold md:text-5xl lg:text-8xl">
      <span v-if="connected" class="truncate">{{ projectName }}</span>
      <span v-else class="text-6xl text-ellipsis italic opacity-50">
        Oops, looks like it's not connected...
      </span>
    </h1>

    <InputText v-model="vibedAddr" />
    <div>{{ wsAddr }}</div>
  </div>
</template>

<script setup lang="ts">
import { inject, ref, watch } from "vue";
import { ProjectInfo, Vibed } from "../App.vue";
import { get } from "@vueuse/core";

const { name: projectName } = inject<ProjectInfo>("project-info")!;

// LYN: Vibed Addr
const { addr, wsAddr, changeAddr, connected } = inject<Vibed>("vibed")!;
const vibedAddr = ref(get(addr));
watch(vibedAddr, (newAddr) => changeAddr(newAddr));
</script>
