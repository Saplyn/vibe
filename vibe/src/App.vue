<template>
  <div class="flex h-screen w-screen flex-col justify-between">
    <PlayControlBar />

    <main class="grow overflow-auto">
      <RouterView />
    </main>

    <nav
      class="border-surface grid grid-flow-col gap-2 rounded-t-lg border-t-4 px-2 pt-2"
      :class="isFullscreen ? '' : 'pb-2'"
    >
      <Button
        v-for="route in $router.getRoutes()"
        :severity="$route.path === route.path ? 'primary' : 'secondary'"
        :class="isFullscreen ? 'rounded-b-none' : ''"
        :pt:label:class="
          $route.path === route.path
            ? 'font-bold'
            : 'font-bold hidden md:inline'
        "
        :label="route.name?.toString()"
        @click="$router.push(route.path)"
      >
        <template #icon>
          <span class="material-symbols-rounded">{{ route.meta.icon }}</span>
        </template>
      </Button>
    </nav>
  </div>
</template>

<script setup lang="ts">
import { getCurrentWindow } from "@tauri-apps/api/window";
import { onMounted, ref } from "vue";

// LYN: Fullscreen Detection
const isFullscreen = ref(false);
onMounted(async () => {
  isFullscreen.value = await getCurrentWindow().isMaximized();

  getCurrentWindow().listen("tauri://resize", async (_) => {
    isFullscreen.value = await getCurrentWindow().isMaximized();
  });
});
</script>
