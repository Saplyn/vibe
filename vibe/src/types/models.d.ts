export type Page<T> = [T, T, T, T];

export type Pattern = {
  name: string;
  page_count: number;
  midi_path: string;
  midi_codes: Page<number | null>[];
  messages: Messages[];
};

export type Messages = {
  payload: MinOscMessage;
  actives: Page<boolean>[];
};

export type Track = {
  name: string;
  active: boolean;
  loop: boolean;
  progress: number | null;
  patterns: string[];
};

export type Event = {
  name: string;
  path: string;
  shortcut: string | null;
  payload: MinOscArg;
};

export type Slider = {
  name: string;
  path: string;
  val: number;
  max: number;
  min: number;
};

export type MinOscMessage = {
  path: string;
  arg: MinOscArg;
};

export type MinOscArg =
  | { type: "Float"; value: number }
  | { type: "String"; value: string };
