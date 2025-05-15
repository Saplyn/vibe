<template>
  <div>
    <div class="flex">
      <ToggleButton
        v-for="(_, i) in 5"
        v-model="actives[i]"
        pt:label:class="hidden"
        on-icon="pi pi-lock"
        off-icon="pi pi-lock-open"
      />
      {{ actives }}
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";

const actives = ref([false, false, false, false, false]);
</script>
