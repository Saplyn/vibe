<template>
  <Splitter
    class="h-full rounded-none border-none bg-transparent"
    layout="vertical"
    :pt:gutterHandle:class="'outline-1 outline-surface-500 bg-surface-500'"
  >
    <SplitterPanel :min-size="30">events</SplitterPanel>

    <SplitterPanel :min-size="40">sliders</SplitterPanel>
  </Splitter>
</template>

<script setup lang="ts"></script>
