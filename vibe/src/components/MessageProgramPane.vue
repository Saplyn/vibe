<template>
  <div class="dark:bg-surface-900 bg-surface-50 h-full">
    <div class="flex">
      <!-- LYN: Midi Quick Peek -->
      <div
        class="text-primary/70 flex w-60 shrink-0 items-center justify-end pr-2 font-mono"
      >
        Midi Peek
      </div>
      <div v-for="(_, pageOffset) in 4" class="flex w-full">
        <div
          class="border-surface-50 dark:border-surface-900 flex h-8 shrink-0 grow items-center justify-center rounded-lg border-4"
          v-for="(_, slot) in 4"
          :class="
            codes?.[startingPage + pageOffset]?.[slot] == null
              ? 'dark:bg-surface-900 bg-surface-50'
              : 'bg-primary/70'
          "
        >
          {{ codes?.[startingPage + pageOffset]?.[slot] ?? "" }}
        </div>
      </div>
    </div>

    <!-- LYN: Message Programming -->
    <div v-for="(msg, index) in messages" class="flex w-full">
      <div class="flex w-60 max-w-60 shrink-0">
        <ButtonGroup class="grow">
          <Button
            :label="`${msg.payload.path}`"
            @click="togglePopover($event, index)"
            class="justify-start rounded-none font-mono"
            :pt:label:class="'truncate max-w-34'"
            fluid
          >
            <template #icon>
              <span class="material-symbols-rounded">
                {{ msgIcon(msg.payload.arg.type) }}
              </span>
            </template>
          </Button>
          <Button
            severity="danger"
            class="rounded-none"
            @click="deleteMsg(index)"
          >
            <template #icon>
              <span class="material-symbols-rounded">delete</span>
            </template>
          </Button>
        </ButtonGroup>
      </div>

      <!-- Valid -->
      <div class="flex grow">
        <div v-for="(_, pageOffset) in 4" class="grow">
          <div v-if="startingPage + pageOffset < pageCount" class="flex grow">
            <ToggleButton
              v-for="(_, slot) in 4"
              v-model="msg.actives[pageOffset][slot]"
              pt:label:class="hidden"
              class="grow"
              :pt:root:class="
                'rounded-none font-mono grow flex items-center justify-center ' +
                ((startingPage + pageOffset) % 2 === 0
                  ? 'dark:bg-surface-900 bg-surface-200'
                  : 'dark:border-surface-900 border-surface-200')
              "
              :pt:content="
                (opt: any) => ({
                  class: opt.context.active
                    ? 'h-full bg-primary text-primary-contrast font-bold'
                    : 'h-full',
                })
              "
            >
              <template #icon>
                <span class="material-symbols-rounded">
                  {{ msg.actives[pageOffset][slot] ? "music_note" : "close" }}
                </span>
              </template>
            </ToggleButton>
          </div>

          <div v-else class="flex grow">
            <ToggleButton
              v-for="_ in 4"
              :disabled="true"
              pt:label:class="hidden"
              class="grow"
              pt:root:class="rounded-none font-mono grow flex items-center justify-center"
              pt:content="h-full"
            >
              <template #icon>
                <span class="material-symbols-rounded">close</span>
              </template>
            </ToggleButton>
          </div>
        </div>
      </div>
    </div>

    <Popover ref="op">
      <div class="flex flex-col gap-2">
        <SelectButton
          v-model="messages![popoverEditingId!].payload.arg.type"
          :options="msgTypeOpts"
        >
          <template #option="slotProps">
            <span class="material-symbols-rounded">
              {{ msgIcon(slotProps.option) }}
            </span>
            {{ slotProps.option }}
          </template>
        </SelectButton>

        <FloatLabel variant="on">
          <InputText
            id="msg-path"
            v-model="messages![popoverEditingId!].payload.path"
          />
          <label for="msg-path">Path</label>
        </FloatLabel>

        <FloatLabel variant="on">
          <InputText
            id="msg-arg"
            v-if="messages![popoverEditingId!].payload.arg.type === 'String'"
            v-model="messages![popoverEditingId!].payload.arg.value as string"
            :invalid="
              !typeMatch(
                messages![popoverEditingId!].payload.arg.value,
                'string',
              )
            "
          />
          <InputNumber
            id="msg-arg"
            v-if="messages![popoverEditingId!].payload.arg.type === 'Float'"
            v-model="messages![popoverEditingId!].payload.arg.value as number"
            :invalid="
              !typeMatch(
                messages![popoverEditingId!].payload.arg.value,
                'number',
              )
            "
          />
          <label for="msg-arg">Arg</label>
        </FloatLabel>
      </div>
    </Popover>
  </div>
</template>

<script setup lang="ts">
import { computed, ref, watch } from "vue";
import { Messages, Page } from "../types/models";
import { get, set } from "@vueuse/core";
import { info } from "@tauri-apps/plugin-log";

const messages = defineModel<Messages[]>("messages");
const valid = defineModel<boolean>("valid");
defineProps<{
  startingPage: number;
  pageCount: number;
  codes: Page<number | null>[];
}>();

// LYN: Popover
const op = ref();
const popoverEditingId = ref<number>();
function togglePopover(event: MouseEvent, id: number) {
  set(popoverEditingId, id);
  op.value.toggle(event);
}

// LYN: Edit Message
const msgTypeOpts = ref(["String", "Float"]);
function typeMatch(value: any, type: string): boolean {
  return typeof value === type;
}
const ok = computed(() => {
  if (messages == undefined) {
    return true;
  }
  let ok = true;
  for (let msg of get(messages)!) {
    switch (msg.payload.arg.type) {
      case "String":
        ok &&= typeof msg.payload.arg.value === "string";
        break;
      case "Float":
        ok &&= typeof msg.payload.arg.value === "number";
        break;
    }
  }
  return ok;
});
watch(ok, (ok) => set(valid, ok));

// LYN: Delete Message
function deleteMsg(index: number) {
  const msgs = get(messages)!;
  msgs.splice(index, 1);
  set(messages, msgs);
}

// LYN: Styling
function msgIcon(type: string): string {
  switch (type) {
    case "String":
      return "notes";
    case "Float":
      return "numbers";
    default:
      return "question_mark";
  }
}
</script>
