import { useWebSocket, UseWebSocketReturn } from "@vueuse/core";
import { ServerCommand } from "../types";
import { computed, ComputedRef } from "vue";

export type UseVibedReturn = {
  ws: UseWebSocketReturn<any>;
  connected: ComputedRef<boolean>;
  command: (cmd: ServerCommand) => void;
};

export function useVibed(): UseVibedReturn {
  const ws = useWebSocket("ws://127.0.0.1:8000", {
    autoReconnect: true,
  });

  const connected = computed(() => ws.status.value === "OPEN");

  function command(cmd: ServerCommand) {
    ws.send(JSON.stringify(cmd));
  }

  return { ws, connected, command };
}
