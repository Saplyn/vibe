<template>
  <div class="dark:bg-surface-900 bg-surface-50 flex h-full flex-col">
    <div class="flex h-full">
      <div class="flex flex-col">
        <Button
          @click="octave++"
          class="h-8 min-h-8 rounded-none"
          :disabled="octave === 9"
        >
          <template #icon>
            <span class="material-symbols-rounded">
              keyboard_double_arrow_up
            </span>
          </template>
        </Button>
        <div
          v-for="i in 12"
          :class="
            isBlackKey(12 - i)
              ? 'bg-surface-950 text-surface-100'
              : 'bg-surface-100 text-surface-950'
          "
          class="flex min-h-[42px] grow items-center justify-center font-bold"
        >
          {{ (octave + 1) * 12 - i }}
        </div>
        <Button
          @click="octave--"
          class="h-8 min-h-8 rounded-none"
          :disabled="octave === 0"
        >
          <template #icon>
            <span class="material-symbols-rounded">
              keyboard_double_arrow_down
            </span>
          </template>
        </Button>
      </div>

      <div v-for="(_, pageOffset) in 4" class="flex grow">
        <!-- Valid -->
        <div
          v-if="startingPage + pageOffset < pageCount"
          v-for="(_, index) in 4"
          class="flex grow flex-col"
        >
          <div
            class="border-surface-50 dark:border-surface-900 flex h-8 shrink-0 items-center justify-center rounded-lg border-4"
            :class="
              noteAbove(pageOffset, index) == undefined
                ? 'dark:bg-surface-900 bg-surface-50'
                : 'bg-primary/70'
            "
          >
            {{ noteAbove(pageOffset, index) }}
          </div>
          <SelectButton
            v-model="codes![startingPage + pageOffset][index]"
            :options="midiOpts"
            class="grow flex-col"
            :pt:pcToggleButton:root:class="
              'rounded-none font-mono grow flex items-center justify-center ' +
              ((startingPage + pageOffset) % 2 === 0
                ? 'dark:bg-surface-900 bg-surface-200'
                : 'dark:border-surface-900 border-surface-200')
            "
            :pt:pcToggleButton:content="
              (opt: any) => ({
                class: opt.context.active
                  ? 'h-full bg-primary text-primary-contrast font-bold'
                  : 'h-full',
              })
            "
          />
          <div
            class="border-surface-50 dark:border-surface-900 flex h-8 shrink-0 items-center justify-center rounded-lg border-4"
            :class="
              noteBelow(pageOffset, index) == undefined
                ? 'dark:bg-surface-900 bg-surface-50'
                : 'bg-primary/70'
            "
          >
            {{ noteBelow(pageOffset, index) }}
          </div>
        </div>

        <!-- Invalid -->
        <div v-else v-for="_ in 4" class="flex grow flex-col">
          <div class="h-8 shrink-0">{{ noteAbove() }}</div>
          <SelectButton
            :disabled="true"
            :options="placeholderOpts"
            class="grow flex-col"
            pt:pcToggleButton:root:class="rounded-none font-mono grow flex items-center justify-center"
            pt:pcToggleButton:content="h-full"
          />
          <div class="h-8 shrink-0">{{ noteBelow() }}</div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, ref } from "vue";
import { Page } from "../types/models";
import { get } from "@vueuse/core";

const codes = defineModel<Page<number | null>[]>("codes");
const props = defineProps<{
  startingPage: number;
  pageCount: number;
}>();

const octave = ref<0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9>(5);
const midiOpts = computed(() =>
  Array.from({ length: 12 }, (_, i) => octave.value * 12 + (11 - i)),
);
const placeholderOpts = Array(12).fill("--");

// LYN: Note Key Styling
function noteAbove(offset?: number, index?: number): number | undefined {
  if (offset != undefined && index != undefined) {
    let note = get(codes)![props.startingPage! + offset][index];
    if (note != undefined && note >= (get(octave) + 1) * 12) {
      return note;
    }
  }
  return undefined;
}
function noteBelow(offset?: number, index?: number): number | undefined {
  if (offset != undefined && index != undefined) {
    let note = get(codes)![props.startingPage! + offset][index];
    if (note != undefined && note < get(octave) * 12) {
      return note;
    }
  }
  return undefined;
}
function isBlackKey(midi: number): boolean {
  return [1, 3, 6, 8, 10].includes(midi % 12);
}
</script>
