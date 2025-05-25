<template>
  <div class="h-full">
    <BlockUI class="flex h-full w-full" :blocked="!connected">
      <div
        class="border-surface flex w-1/4 max-w-96 min-w-64 shrink-0 flex-col justify-between border-r-4"
      >
        <!-- LYN: Pattern List -->
        <div class="m-2 flex grow flex-col gap-2 overflow-auto">
          <ButtonGroup v-for="pattern in patterns">
            <Button
              fluid
              class="justify-between"
              :pt:label:class="
                'font-mono ' +
                (pattern.name === editingName ? 'font-black' : '')
              "
              :label="pattern.name"
              :badge="pattern.page_count.toString()"
              :badge-severity="
                pattern.name === editingName ? 'contrast' : 'secondary'
              "
              @click="setEditing(pattern.name)"
              :severity="pattern.name === editingName ? 'primary' : 'secondary'"
            />
            <Button
              @click="confirmDelPattern($event, pattern.name)"
              :severity="pattern.name === editingName ? 'danger' : 'secondary'"
              :class="pattern.name === editingName ? '' : 'text-red-400'"
            >
              <template #icon>
                <span class="material-symbols-rounded">delete</span>
              </template>
            </Button>
          </ButtonGroup>
        </div>

        <!-- LYN: Add New Pattern -->
        <div class="border-surface flex gap-2 border-t-4 border-dotted p-2">
          <FloatLabel class="grow" variant="in">
            <InputText
              fluid
              :disabled="!connected"
              v-model="patternNameToAdd"
            />
            <label>New pattern name</label>
          </FloatLabel>

          <Button
            class="w-14"
            @click="addPatternWrapper()"
            :disabled="!connected || patternNameToAdd == ''"
            :ref="addPatternButtonRef"
          >
            <template #icon>
              <span class="material-symbols-rounded">music_note_add</span>
            </template>
          </Button>
        </div>
      </div>

      <div class="flex h-full w-full flex-col overflow-auto">
        <!-- LYN: Pane Conrtol -->
        <div
          class="border-surface dark:bg-surface-900 bg-surface-50 sticky top-0 left-0 z-50 flex min-h-[62px] gap-2 overflow-auto border-b-4 p-2"
        >
          <SelectButton
            :allow-empty="false"
            v-model="visiblePane"
            :options="programPanes"
            option-label="value"
            data-key="value"
            aria-labelledby="custom"
            :disabled="notEditing"
          >
            <template #option="slotProps">
              <span class="material-symbols-rounded">
                {{ slotProps.option.icon }}
              </span>
            </template>
          </SelectButton>

          <!-- LYN: Page Size -->
          <span class="h-10 w-20">
            <FloatLabel variant="on">
              <InputNumber
                id="page-count"
                fluid
                v-if="!notEditing"
                v-model="patternEditing!.page_count"
                showButtons
                :min="0"
              />
              <InputNumber v-else id="page-count" fluid disabled />
              <label for="page-count">Pages</label>
            </FloatLabel>
          </span>

          <Divider layout="vertical" />

          <!-- LYN: Page Control -->
          <Button
            :disabled="notEditing || startingPage === 0"
            @click="startingPage--"
          >
            <template #icon>
              <span class="material-symbols-rounded">
                keyboard_double_arrow_left
              </span>
            </template>
          </Button>

          <div class="flex w-4 items-center justify-center">
            {{ startingPage }}
          </div>

          <Button
            :disabled="notEditing || startingPage >= patternEditing!.page_count"
            @click="startingPage++"
          >
            <template #icon>
              <span class="material-symbols-rounded">
                keyboard_double_arrow_right
              </span>
            </template>
          </Button>

          <Divider layout="vertical" />

          <!-- LYN: Edit Control -->
          <ToggleButton
            on-label="Live"
            off-label="Manual"
            v-model="liveEditing"
            :pt:content="
              (opt: any) => ({
                class: opt.context.active
                  ? 'bg-primary text-primary-contrast'
                  : '',
              })
            "
          />
          <Button
            :disabled="!dirty || !validMessages || liveEditing"
            :variant="
              editingName != undefined && dirty && validMessages
                ? ''
                : 'outlined'
            "
            :severity="validMessages ? '' : 'danger'"
            label="Make Edit"
            @click="makeEdit()"
          >
            <template #icon>
              <span class="material-symbols-rounded">edit_square</span>
            </template>
          </Button>

          <Divider layout="vertical" />

          <!-- LYN: Midi Path -->
          <span class="h-10" v-if="visiblePane.value === 'midi'">
            <InputGroup>
              <InputGroupAddon>
                <span class="material-symbols-rounded">lyrics</span>
              </InputGroupAddon>
              <FloatLabel variant="on">
                <InputText
                  id="midi-path"
                  fluid
                  v-if="!notEditing"
                  v-model="patternEditing!.midi_path"
                />
                <InputNumber v-else id="midi-path" fluid disabled />
                <label for="page-count">Midi Path</label>
              </FloatLabel>
            </InputGroup>
          </span>

          <Button
            v-if="visiblePane.value === 'message'"
            @click="addNewMessage"
            :disabled="notEditing"
            label="New Message"
          >
            <template #icon>
              <span class="material-symbols-rounded">add_comment</span>
            </template>
          </Button>
        </div>

        <!-- LYN: Programming Pane -->
        <div
          v-if="notEditing"
          class="text-primary/50 flex h-full items-center justify-center text-5xl italic"
        >
          Select a pattern to edit...
        </div>
        <div v-else class="flex h-full flex-col">
          <!-- LYN: Midi Programming -->
          <MidiProgramPane
            v-if="visiblePane.value === 'midi'"
            v-model:codes="patternEditing!.midi_codes"
            :page-count="patternEditing!.page_count"
            :starting-page="startingPage"
          />

          <!-- LYN: Message Programming -->
          <MessageProgramPane
            v-if="visiblePane.value === 'message'"
            v-model:messages="patternEditing!.messages"
            v-model:valid="validMessages"
            :codes="patternEditing!.midi_codes"
            :page-count="patternEditing!.page_count"
            :starting-page="startingPage"
          />
        </div>
      </div>
    </BlockUI>
  </div>
</template>

<script setup lang="ts">
import { computed, inject, ref, watch } from "vue";
import { PatternEditing, PatternState, Vibed } from "../App.vue";
import { get, onKeyStroke, set, useFocus } from "@vueuse/core";
import { ButtonGroup, useConfirm } from "primevue";
import { Pattern } from "../types/models";
import { cloneDeep, isEqual } from "lodash";

const programPanes = [
  { value: "midi", icon: "piano" },
  { value: "message", icon: "rate_review" },
];
const visiblePane = ref<{ value: string; icon?: string }>({ value: "midi" });

const { connected } = inject<Vibed>("vibed")!;
const { patterns, addPattern, delPattern, editPattern } =
  inject<PatternState>("pattern-state")!;

// LYN: Pattern Editing
const { name: editingName, change: setEditing } =
  inject<PatternEditing>("pattern-editing")!;
const patternOriginal = ref<Pattern>();
const patternEditing = ref<Pattern>();
watch(
  editingName,
  (name) => {
    if (name != undefined) {
      let pattern = get(patterns)?.[name];
      if (pattern != undefined) {
        set(patternOriginal, cloneDeep(pattern));
        set(patternEditing, cloneDeep(pattern));
      } else {
        set(patternEditing, undefined);
      }
    } else {
      set(patternEditing, undefined);
    }
  },
  { immediate: true },
);
watch(
  () => {
    let name = get(editingName);
    if (name != undefined) {
      let pat = get(patterns)?.[name];
      if (pat != undefined) {
        return [pat, pat.messages.length];
      }
    }
    return [undefined, undefined];
  },
  ([pat, _]) => {
    if (pat != undefined) {
      set(patternOriginal, cloneDeep(pat));
    } else {
      set(patternOriginal, undefined);
    }
  },
);
const notEditing = computed(() => {
  return editingName == undefined || get(patternEditing) == undefined;
});
const dirty = computed(() => {
  return !isEqual(get(patternOriginal), get(patternEditing));
});

// LYN: Delete Pattern
const confirm = useConfirm();
function confirmDelPattern(event: MouseEvent, name: string) {
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
    accept: () => {
      delPattern(name);
    },
  });
}

// LYN: Make Page Edit
const validMessages = ref(true);
function makeEdit() {
  if (!get(notEditing)) {
    editPattern(get(editingName)!, get(patternEditing)!);
  }
}

// LYN: Page Size Syncing
watch(
  () => get(patternEditing)?.page_count,
  (count) => {
    if (count != undefined) {
      let midi_codes = get(patternEditing)!.midi_codes;
      if (midi_codes.length < count) {
        for (let i = midi_codes.length; i < count; i++) {
          midi_codes.push([null, null, null, null]);
        }
      } else if (midi_codes.length > count) {
        midi_codes.splice(count);
      }

      for (let msg of get(patternEditing)!.messages) {
        if (msg.actives.length < count) {
          for (let i = msg.actives.length; i < count; i++) {
            msg.actives.push([false, false, false, false]);
          }
        } else if (msg.actives.length > count) {
          msg.actives.splice(count);
        }
      }
    }
  },
);

// LYN: Page Scrolling
const startingPage = ref(0);
watch(
  () => get(patternEditing)?.page_count,
  (count) => {
    if (count != undefined && get(startingPage) >= count) {
      set(startingPage, count);
    }
  },
);

// LYN: Add Pattern
const patternNameToAdd = ref<string>("");
function addPatternWrapper() {
  addPattern(get(patternNameToAdd));
  set(patternNameToAdd, "");
}
const addPatternButtonRef = ref();
const { focused: addPatternButtonFocused } = useFocus(addPatternButtonRef);
onKeyStroke(
  "Enter",
  (e) => {
    if (addPatternButtonFocused && connected && get(patternNameToAdd) != "") {
      addPatternWrapper();
      e.preventDefault();
    }
  },
  { dedupe: true },
);

// LYN: Add New Message
function addNewMessage() {
  let pat = get(patternEditing)!;
  pat.messages.push({
    actives: Array.from({ length: pat.page_count }, () => [
      false,
      false,
      false,
      false,
    ]),
    payload: {
      path: "/",
      arg: {
        type: "String",
        value: "",
      },
    },
  });
}

// LYN: Live Edit
const liveEditing = ref(false);
watch(dirty, () => {
  if (get(liveEditing) && get(dirty) && get(validMessages)) {
    makeEdit();
  }
});
</script>
