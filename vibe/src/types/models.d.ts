export type Page<T> = [T, T, T, T];

export type Pattern = {
  page_count: number;
  midi_path: string;
  midi_codes: Page<number | null>[];
  messages: Page<Messages>[];
};

export type Messages = {
  payload: MinOscMessage;
  active: Page<boolean>[];
};

export type Track = {
  active: boolean;
  patterns: string[];
};

export type MinOscMessage = {
  path: string;
  args: MinOscArg[];
};

type MinOscArg =
  | { type: "Float"; value: number }
  | { type: "String"; value: string };
