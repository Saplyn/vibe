<template>
  <div class="h-full">
    <div class="flex h-full w-full">
      <!-- LYN: Pattern List -->
      <div class="border-surface w-1/4 max-w-96 min-w-64 shrink-0 border-r-4">
        pattern list
      </div>

      <div class="flex h-full w-full flex-col overflow-auto">
        <!-- LYN: Pane Conrtol -->
        <div class="border-surface flex gap-2 border-b-4 p-2">
          <SelectButton
            :allow-empty="false"
            v-model="visiblePane"
            :options="programPanes"
            option-label="value"
            data-key="value"
            aria-labelledby="custom"
          >
            <template #option="slotProps">
              <span class="material-symbols-rounded">
                {{ slotProps.option.icon }}
              </span>
            </template>
          </SelectButton>

          <Button>
            <template #icon>
              <span class="material-symbols-rounded">
                keyboard_double_arrow_left
              </span>
            </template>
          </Button>

          <Button>
            <template #icon>
              <span class="material-symbols-rounded">
                keyboard_double_arrow_right
              </span>
            </template>
          </Button>
        </div>

        <!-- LYN: Program Pane -->
        <div class="h-full overflow-auto">
          <MidiProgramPane v-if="visiblePane.value === 'midi'" />

          <MessageProgramPane v-if="visiblePane.value === 'message'" />
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";

const programPanes = [
  { value: "midi", icon: "piano" },
  { value: "message", icon: "rate_review" },
];
const visiblePane = ref<{ value: string; icon?: string }>({ value: "midi" });
</script>
