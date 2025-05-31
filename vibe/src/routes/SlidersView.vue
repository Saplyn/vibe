<template>
  <BlockUI class="flex h-full w-full" :blocked="!connected">
    <div class="flex h-full w-full flex-col justify-between">
      <VirtualScroller
        :items="sliderArray"
        :item-size="253"
        lazy
        orientation="horizontal"
        class="!h-full !w-full"
        pt:content:class="flex grow gap-2 p-2"
      >
        <template #item="{ item: slider }">
          <div
            class="dark:bg-surface-800 bg-surface-100 border-surface flex flex-col justify-between gap-2 rounded-lg border-2 p-2"
            :style="
              slider.color != null
                ? `border-color: ${slider.color} !important;`
                : ''
            "
          >
            <!-- LYN: Name -->
            <div class="flex items-center justify-center p-4 font-mono">
              {{ slider.name }}
            </div>

            <!-- LYN: Slider -->
            <div class="flex h-1/2 justify-around gap-2 p-2">
              <div class="flex flex-col items-center justify-between gap-4">
                <Slider
                  orientation="vertical"
                  class="w-4 grow"
                  pt:handle:class="bg-primary"
                  :max="slider.max"
                  :min="slider.min"
                  :step="(slider.max - slider.min) / 1000"
                  v-model="slider.val"
                  @update:model-value="updateVal(slider.name, slider.val)"
                />
                <div
                  class="dark:bg-surface-950 bg-surface-0 flex w-24 items-center justify-center rounded-lg p-1"
                >
                  {{ slider.val }}
                </div>
              </div>

              <div class="flex w-24 flex-col justify-between gap-2">
                <InputNumber
                  show-buttons
                  button-layout="vertical"
                  v-model="slider.max"
                  @update:model-value="updateSlider(slider.name, slider)"
                />
                <InputNumber
                  show-buttons
                  button-layout="vertical"
                  v-model="slider.min"
                  @update:model-value="updateSlider(slider.name, slider)"
                />
                <Button
                  severity="danger"
                  variant="outlined"
                  @click="confirmDelSlider($event, slider.name)"
                >
                  <span class="material-symbols-rounded">delete</span>
                </Button>
              </div>
            </div>

            <div class="flex flex-col gap-2">
              <!-- LYN: Path -->
              <FloatLabel variant="on">
                <InputText
                  v-model="slider.path"
                  @update:model-value="updateSlider(slider.name, slider)"
                />
                <label>Path</label>
              </FloatLabel>

              <!-- LYN: Color -->
              <FloatLabel variant="on">
                <InputText
                  v-model="slider.color"
                  @update:model-value="updateSlider(slider.name, slider)"
                />
                <label>Color</label>
              </FloatLabel>
            </div>
          </div>
        </template>
      </VirtualScroller>

      <!-- LYN: Add Slider -->
      <div class="border-surface flex gap-2 border-t-4 border-dotted px-4 py-2">
        <Button
          @click="addSliderWrapper()"
          label="Add New Slider"
          :disabled="sliderNameToAdd == ''"
        >
          <template #icon>
            <span class="material-symbols-rounded">playlist_add</span>
          </template>
        </Button>
        <FloatLabel class="grow" variant="on">
          <InputText
            fluid
            :disabled="!connected"
            v-model="sliderNameToAdd"
            ref="addSliderButtonRef"
          />
          <label>New slider name</label>
        </FloatLabel>
      </div>
    </div>
  </BlockUI>
</template>

<script setup lang="ts">
import { inject, ref, computed, watch } from "vue";
import VirtualScroller from "primevue/virtualscroller"; // Import VirtualScroller
import { SliderState, Vibed } from "../App.vue";
import { get, onKeyStroke, set, useFocus } from "@vueuse/core";
import { Slider as SliderModel } from "../types/models";
import { cloneDeep } from "lodash";
import { useConfirm } from "primevue";

const { connected } = inject<Vibed>("vibed")!;
const {
  sliders: readonlySliders,
  addSlider,
  editSlider,
  delSlider,
  setSliderVal,
} = inject<SliderState>("slider-state")!;

// LYN: Sliders
const addSliderButtonRef = ref();
const { focused: addSliderButtonFocused } = useFocus(addSliderButtonRef);
const sliderNameToAdd = ref("");
onKeyStroke(
  "Enter",
  (e) => {
    if (addSliderButtonFocused && connected && get(sliderNameToAdd) != "") {
      addSliderWrapper();
      e.preventDefault();
    }
  },
  { dedupe: true },
);
function addSliderWrapper() {
  addSlider(get(sliderNameToAdd));
  set(sliderNameToAdd, "");
}
const sliders = ref<Record<string, SliderModel>>();
watch(
  readonlySliders,
  (readonlySliders) => {
    set(sliders, cloneDeep(readonlySliders));
  },
  { immediate: true },
);
function updateVal(name: string, newVal: number) {
  setSliderVal(name, newVal);
}
function updateSlider(name: string, slider: SliderModel) {
  editSlider(name, slider);
}
const confirm = useConfirm();
function confirmDelSlider(event: MouseEvent, name: string) {
  confirm.require({
    target: event.currentTarget as any,
    message: "Confirm deletion?",
    icon: "pi pi-info-circle",
    rejectProps: {
      label: "Cancel",
      severity: "secondary",
      outlined: true,
    },
    acceptProps: {
      label: "Delete",
      severity: "danger",
    },
    accept: () => delSlider(name),
  });
}
const sliderArray = computed<SliderModel[]>(() =>
  Object.values(get(sliders) ?? {}),
);
</script>
