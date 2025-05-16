import { Track, Pattern } from "./models";

export type ServerCommand =
  | { action: "SetProjectName"; payload: { name: string } }
  | { action: "CommChangeAddr"; payload: { addr: string } }
  | { action: "CtrlChangeContext"; payload: { context: string | null } }
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
  | { action: "RequestProjectName" }
  | { action: "RequestCommAddr" }
  | { action: "RequestCtrlContext" }
  | { action: "RequestTrack"; payload: { name: string } }
  | { action: "RequestAllTracks" }
  | { action: "RequestPattern"; payload: { name: string } }
  | { action: "RequestAllPatterns" }
  | { action: "RequestTickerBpm" }
  | { action: "RequestTickerPlaying" }
  | { action: "RequestTickerTick" };

export type ClientCommand =
  | { action: "ProjectNameUpdated"; payload: { name: string } }
  | { action: "CommAddrChanged"; payload: { addr: string } }
  | { action: "CtrlContextChanged"; payload: { context: string | null } }
  | { action: "TrackAdded"; payload: { name: string; track: Track } }
  | { action: "TrackDeleted"; payload: { name: string } }
  | { action: "TrackEdited"; payload: { name: string; track: Track } }
  | { action: "PatternAdded"; payload: { name: string; pattern: Pattern } }
  | { action: "PatternDeleted"; payload: { name: string } }
  | { action: "PatternEdited"; payload: { name: string; pattern: Pattern } }
  | { action: "TickerBpmUpdated"; payload: { bpm: number } }
  | { action: "TickerTick"; payload: { tick: number } }
  | { action: "TickerPlaying" }
  | { action: "TickerPaused" }
  | { action: "TickerStopped" }
  | { action: "ResponseProjectName"; payload: { name: string } }
  | { action: "ResponseCommAddr"; payload: { addr: string } }
  | { action: "ResponseCtrlContext"; payload: { context: string | null } }
  | { action: "ResponseTrack"; payload: { name: string; track: Track } }
  | { action: "ResponseAllTracks"; payload: { tracks: Record<string, Track> } }
  | { action: "ResponsePattern"; payload: { name: string; pattern: Pattern } }
  | {
      action: "ResponseAllPatterns";
      payload: { patterns: Record<string, Pattern> };
    }
  | { action: "ResponseTickerBpm"; payload: { bpm: number } }
  | { action: "ResponseTickerPlaying"; payload: { playing: boolean } }
  | { action: "ResponseTickerTick"; payload: { tick: number } };
