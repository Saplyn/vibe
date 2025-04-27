<template>
  <div class="flex flex-col h-screen w-screen justify-between">
    <main class="overflow-auto grow">
      <RouterView />
    </main>

    <nav class="text-primary-contrast grid grid-flow-col gap-4">
      <RouterLink class="bg-primary" to="/">Project</RouterLink>
      <RouterLink class="bg-primary" to="/tracks">Tracks</RouterLink>
      <RouterLink class="bg-primary" to="/patterns">Patterns</RouterLink>
    </nav>
  </div>
</template>

<script setup lang="ts"></script>

