<template>
  <div class="flex h-full flex-col items-center justify-center">
    <h1 class="text-primary m-8 text-2xl font-bold md:text-5xl lg:text-8xl">
      <span v-if="connected" class="truncate">{{ projectName }}</span>
      <span v-else class="text-6xl text-ellipsis italic opacity-50">
        Oops, looks like it's not connected...
      </span>
    </h1>

    <Button
      size="large"
      :variant="editVisible ? '' : 'outlined'"
      @click="editVisible = !editVisible"
    >
      <template #icon>
        <span class="material-symbols-rounded">edit_square</span>
      </template>
    </Button>

    <div v-if="editVisible" class="mt-4 flex flex-col gap-2">
      <!-- LYN: Vibed Server -->
      <InputGroup>
        <InputGroupAddon class="flex gap-2">
          <span class="material-symbols-rounded">dns</span>
        </InputGroupAddon>
        <InputText v-model="vibedAddr" />
        <InputGroupAddon class="flex gap-2">
          <span>
            <span class="font-mono">vibed</span>
            Server
          </span>
        </InputGroupAddon>
      </InputGroup>

      <!-- LYN: Project Name -->
      <InputGroup>
        <InputGroupAddon class="flex gap-2">
          <span class="material-symbols-rounded">signature</span>
        </InputGroupAddon>
        <InputText v-model="projectName" :disabled="!connected" />
        <InputGroupAddon class="flex gap-2">
          <span>Project Name</span>
        </InputGroupAddon>
      </InputGroup>

      <!-- LYN: TCP Server -->
      <InputGroup>
        <InputGroupAddon class="flex gap-2">
          <span class="material-symbols-rounded">radio</span>
        </InputGroupAddon>
        <InputText v-model="commAddr" :disabled="!connected" />
        <InputGroupAddon class="flex gap-2">
          <span>Remote Server</span>
        </InputGroupAddon>
      </InputGroup>
    </div>
  </div>
</template>

<script setup lang="ts">
import { inject, ref, watch } from "vue";
import { CommInfo, ProjectInfo, Vibed } from "../App.vue";
import { get, set } from "@vueuse/core";

// LYN: Communication Addr
const { addr: injectedCommAddr, change: changeCommAddr } =
  inject<CommInfo>("comm-info")!;
const commAddr = ref("");
watch(injectedCommAddr, (newAddr) => set(commAddr, get(newAddr)), {
  immediate: true,
});
watch(commAddr, (newAddr) => {
  if (newAddr != undefined) {
    changeCommAddr(newAddr);
  }
});

const editVisible = ref(false);

// LYN: Vibed Addr
const { addr, changeAddr, connected } = inject<Vibed>("vibed")!;
const vibedAddr = ref(get(addr));
watch(vibedAddr, (newAddr) => changeAddr(newAddr));

// LYN: Project Name
const { name, change: changeName } = inject<ProjectInfo>("project-info")!;
const projectName = ref("");
watch(name, (newName) => set(projectName, get(newName)), {
  immediate: true,
});
watch(projectName, (newName) => {
  if (newName != undefined) {
    changeName(newName);
  }
});
</script>
