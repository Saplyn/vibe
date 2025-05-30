<template>
  <BlockUI class="flex h-full w-full" :blocked="!connected">
    <Splitter
      class="grow overflow-auto rounded-none border-none bg-transparent"
      layout="vertical"
      :pt:gutterHandle:class="'outline-1 outline-surface-500 bg-surface-500'"
    >
      <!-- LYN: Events -->
      <SplitterPanel
        :min-size="40"
        :size="45"
        class="flex flex-col overflow-auto"
      >
        <div
          class="border-surface flex gap-2 border-b-4 border-dotted px-4 py-2"
        >
          <!-- LYN: Add Event -->
          <Button
            label="Add New Event"
            @click="addEventWrapper()"
            :disabled="!connected || eventNameToAdd == ''"
            :ref="addEventButtonRef"
          >
            <template #icon>
              <span class="material-symbols-rounded">library_add</span>
            </template>
          </Button>

          <FloatLabel class="grow" variant="on">
            <InputText fluid :disabled="!connected" v-model="eventNameToAdd" />
            <label>New event name</label>
          </FloatLabel>
        </div>

        <!-- LYN: Event List -->
        <div class="flex h-full flex-wrap gap-2 overflow-auto p-2">
          <div
            class="flex h-1/3 min-h-30 w-1/12 flex-col"
            v-for="event in events"
          >
            <Button
              class="min-h-10 grow rounded-b-none"
              @click="fireEvent(event.name)"
            >
              <div class="flex flex-col font-mono">
                <div class="pt-2 text-xl">{{ event.name }}</div>
                <kbd
                  class="border-primary-500 rounded-lg border-2"
                  v-if="event.shortcut != null && event.shortcut != ''"
                >
                  {{ event.shortcut }}
                </kbd>
              </div>
            </Button>

            <Button
              @click="toggleEventPopover($event, event.name)"
              class="w-full rounded-t-none"
              variant="outlined"
            >
              <template #icon>
                <span class="material-symbols-rounded">rate_review</span>
              </template>
            </Button>
          </div>
        </div>
      </SplitterPanel>

      <!-- LYN: Sliders -->
      <SplitterPanel
        :min-size="40"
        :size="55"
        class="flex flex-col justify-between overflow-auto"
      >
        <div class="flex grow gap-2 overflow-auto p-2">
          <div
            v-for="slider in sliders"
            class="dark:bg-surface-800 bg-surface-100 flex min-h-96 flex-col gap-2 rounded-lg p-2"
          >
            <div class="flex grow items-center justify-center font-mono">
              {{ slider.name }}
            </div>

            <div class="flex grow justify-around gap-2 p-2">
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

            <FloatLabel class="grow" variant="on">
              <InputText
                v-model="slider.path"
                @update:model-value="updateSlider(slider.name, slider)"
              />
              <label>Path</label>
            </FloatLabel>
          </div>
        </div>

        <div
          class="border-surface flex gap-2 border-t-4 border-dotted px-4 py-2"
        >
          <Button @click="addSliderWrapper()" label="Add New Slider">
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
      </SplitterPanel>
    </Splitter>

    <!-- LYN: Edit Event -->
    <Popover ref="eventPopover">
      <div class="flex flex-col gap-2">
        <FloatLabel variant="on">
          <InputText v-model="eventEditing!.shortcut" />
          <label>Shortcut</label>
        </FloatLabel>

        <SelectButton
          v-model="eventEditing!.payload.type"
          :options="eventTypeOpts"
          class="flex items-center justify-center"
        >
          <template #option="slotProps">
            <span class="material-symbols-rounded">
              {{ eventTypeIcon(slotProps.option) }}
            </span>
            {{ slotProps.option }}
          </template>
        </SelectButton>

        <FloatLabel variant="on">
          <InputText id="msg-path" v-model="eventEditing!.path" />
          <label for="msg-path">Path</label>
        </FloatLabel>

        <FloatLabel variant="on">
          <InputText
            id="event-payload"
            v-if="eventEditing!.payload.type === 'String'"
            v-model="eventEditing!.payload.value as string"
            :invalid="!typeMatch(eventEditing!.payload.value, 'string')"
          />
          <InputNumber
            id="event-payload"
            v-if="eventEditing!.payload.type === 'Float'"
            v-model="eventEditing!.payload.value as number"
            :invalid="!typeMatch(eventEditing!.payload.value, 'number')"
          />
          <label for="event-payload">Payload</label>
        </FloatLabel>

        <div class="flex gap-2">
          <Button
            @click="delEventWrapper(eventEditing?.name)"
            class="grow"
            severity="danger"
          >
            <template #icon>
              <span class="material-symbols-rounded">delete</span>
            </template>
          </Button>
          <Button
            label="Make Edit"
            @click="eventMakeEdit()"
            :disabled="!eventDirty || !validEvent"
            :variant="eventDirty && validEvent ? '' : 'outlined'"
            :severity="validEvent ? '' : 'danger'"
          >
            <template #icon>
              <span class="material-symbols-rounded">edit_square</span>
            </template>
          </Button>
        </div>
      </div>
    </Popover>
    <div>{{ shortcutKeys }}</div>
  </BlockUI>
</template>

<script setup lang="ts">
import { computed, inject, ref, watch } from "vue";
import { EventState, SliderState, Vibed } from "../App.vue";
import { get, onKeyStroke, set, useFocus } from "@vueuse/core";
import { Event, Slider as SliderModel } from "../types/models";
import { cloneDeep, isEqual } from "lodash";
import { useConfirm } from "primevue";

const { connected } = inject<Vibed>("vibed")!;
const { events, addEvent, editEvent, delEvent, fireEvent } =
  inject<EventState>("event-state")!;
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
const sliders = ref();
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

// LYN: Event
const addEventButtonRef = ref();
const { focused: addEventButtonFocused } = useFocus(addEventButtonRef);
const eventNameToAdd = ref("");
onKeyStroke(
  "Enter",
  (e) => {
    if (addEventButtonFocused && connected && get(eventNameToAdd) != "") {
      addEventWrapper();
      e.preventDefault();
    }
  },
  { dedupe: true },
);
function addEventWrapper() {
  addEvent(get(eventNameToAdd));
  set(eventNameToAdd, "");
}

// LYN: Edit Event
const eventTypeOpts = ref(["String", "Float"]);
function eventTypeIcon(type: string): string {
  switch (type) {
    case "String":
      return "notes";
    case "Float":
      return "numbers";
    default:
      return "question_mark";
  }
}
function typeMatch(value: any, type: string): boolean {
  return typeof value === type;
}
const validEvent = computed(() => {
  let event = get(eventEditing);
  if (event == undefined) {
    return true;
  }
  let valid = true;
  switch (event.payload.type) {
    case "String":
      valid &&= typeof event.payload.value === "string";
      break;
    case "Float":
      valid &&= typeof event.payload.value === "number";
      break;
  }
  return valid;
});
const eventDirty = computed(() => {
  return !isEqual(get(eventOriginal), get(eventEditing));
});
const eventPopover = ref();
const editingEventName = ref<string>();
const eventOriginal = ref<Event>();
const eventEditing = ref<Event>();
function toggleEventPopover(event: MouseEvent, name: string) {
  set(editingEventName, name);
  eventPopover.value.toggle(event);
}
watch(
  editingEventName,
  (name) => {
    if (name != undefined) {
      let event = get(events)?.[name];
      if (event != undefined) {
        set(eventOriginal, cloneDeep(event));
        set(eventEditing, cloneDeep(event));
      } else {
        set(eventEditing, undefined);
      }
    } else {
      set(eventEditing, undefined);
    }
  },
  { immediate: true },
);
function eventMakeEdit() {
  let name = get(editingEventName);
  if (name != undefined) {
    editEvent(name, get(eventEditing)!);
  }
  eventPopover.value.toggle(false);
  set(editingEventName, undefined);
}
function delEventWrapper(name?: string) {
  if (name == undefined) {
    return;
  }
  eventPopover.value.toggle(false);
  delEvent(name);
  set(eventEditing, undefined);
}

// LYN: Event Shortcut
const shortcutKeys = computed(() => {
  return Object.values(get(events) ?? {}).map((event) => event.shortcut);
});
onKeyStroke(
  (_: KeyboardEvent) => {
    return true;
  },
  (e) => {
    if (get(connected)) {
      const event = Object.values(get(events) ?? {}).find(
        (event) => event.shortcut === e.key,
      );
      if (event) {
        fireEvent(event.name);
      }
    }
  },
  { dedupe: true },
);
</script>
