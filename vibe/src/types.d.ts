type ServerCommand =
  | { action: "SetProjectName"; payload: { name: string } }
  | { action: "CommChangeAddr"; payload: { addr: string } }
  | { action: "CommChangeContext"; payload: { context: string | null } }
  | { action: "TrackAdd"; payload: { name: string } }
  | { action: "TrackDelete"; payload: { name: string } }
  | { action: "TrackEdit"; payload: { name: string; track: Track } }
  | { action: "PatternAdd"; payload: { name: string } }
  | { action: "PatternDelete"; payload: { name: string } }
  | { action: "PatternEdit"; payload: { name: string; pattern: Pattern } }
  | { action: "TickerSetBpm"; payload: { bpm: number } }
  | { action: "TickerPlay" }
  | { action: "TickerPause" }
  | { action: "TickerStop" }
  | { action: "RequstProjectName" }
  | { action: "RequestCommAddr" }
  | { action: "RequestTrack"; payload: { name: string } }
  | { action: "RequestAllTracks" }
  | { action: "RequestPattern"; payload: { name: string } }
  | { action: "RequestAllPatterns" }
  | { action: "RequstTickerBpm" }
  | { action: "RequestTickerCycle" }
  | { action: "RequestTickerState" }
  | { action: "RequestTickerTick" };

type ClientCommand =
  | { action: "ProjectNameUpdated"; payload: { name: string } }
  | { action: "CommAddrChanged"; payload: { addr: string } }
  | { action: "CommContextChanged"; payload: { context: string | null } }
  | { action: "TrackAdded"; payload: { name: string; track: Track } }
  | { action: "TrackDeleted"; payload: { name: string } }
  | { action: "TrackEdited"; payload: { name: string; track: Track } }
  | { action: "PatternAdded"; payload: { name: string; pattern: Pattern } }
  | { action: "PatternDeleted"; payload: { name: string } }
  | { action: "PatternEdited"; payload: { name: string; pattern: Pattern } }
  | { action: "TickerBpmUpdated"; payload: { bpm: number } }
  | { action: "TickerTick"; payload: { index: number } }
  | { action: "TickerPlaying" }
  | { action: "TickerPaused" }
  | { action: "TickerStopped" }
  | { action: "ResponseProjectName"; payload: { string: string } }
  | { action: "ResponseCommAddr"; payload: { string: string } }
  | { action: "ResponseTrack"; payload: { name: string; track: Track } }
  | { action: "ResponseAllTracks"; payload: { tracks: [string, Track][] } }
  | { action: "ResponsePattern"; payload: { name: string; pattern: Pattern } }
  | {
      action: "ResponseAllPatterns";
      payload: { patterns: [string, Pattern][] };
    }
  | { action: "ResponseTickerBpm"; payload: { bpm: number } }
  | { action: "ResponseTickerCycle"; payload: { cycle: number | null } }
  | { action: "ResponseTickerState"; payload: { playing: boolean } }
  | { action: "ResponseTickerTick"; payload: { index: number } };

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
