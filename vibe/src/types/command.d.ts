import { Track, Pattern } from "./models";

export type ServerCommand =
  | { action: "SetProjectName"; payload: { name: string } }
  | { action: "CommChangeAddr"; payload: { addr: string } }
  | { action: "CtrlChangeContext"; payload: { context: string | null } }
  // LYN: Track
  | { action: "TrackAdd"; payload: { name: string } }
  | { action: "TrackDelete"; payload: { name: string } }
  | { action: "TrackEdit"; payload: { name: string; track: Track } }
  | {
      action: "TrackMakeActive";
      payload: { name: string; active: boolean; force: boolean };
    }
  | { action: "TrackMakeLoop"; payload: { name: string; loop: boolean } }
  // LYN: Pattern
  | { action: "PatternAdd"; payload: { name: string } }
  | { action: "PatternDelete"; payload: { name: string } }
  | { action: "PatternEdit"; payload: { name: string; pattern: Pattern } }
  // LYN: Ticker
  | { action: "TickerPlay" }
  | { action: "TickerPause" }
  | { action: "TickerStop" }
  | { action: "TickerSetBpm"; payload: { bpm: number } }
  // LYN: Request
  | { action: "RequestTickerBpm" }
  | { action: "RequestTickerPlaying" }
  | { action: "RequestTickerTick" }
  | { action: "RequestProjectName" }
  | { action: "RequestCommAddr" }
  | { action: "RequestCommStatus" }
  | { action: "RequestCtrlContext" }
  | { action: "RequestTrack"; payload: { name: string } }
  | { action: "RequestAllTracks" }
  | { action: "RequestPattern"; payload: { name: string } }
  | { action: "RequestAllPatterns" };

export type ClientCommand =
  | { action: "ProjectNameUpdated"; payload: { name: string } }
  | { action: "CommAddrChanged"; payload: { addr: string } }
  | { action: "CommStatusChanged"; payload: { established: boolean } }
  | { action: "CtrlContextChanged"; payload: { context: string | null } }
  // LYN: Track
  | { action: "TrackAdded"; payload: { name: string; track: Track } }
  | { action: "TrackDeleted"; payload: { name: string } }
  | { action: "TrackEdited"; payload: { name: string; track: Track } }
  | { action: "TrackMadeActive"; payload: { name: string; active: boolean } }
  | { action: "TrackMadeLoop"; payload: { name: string; loop: boolean } }
  | {
      action: "TrackProgressUpdate";
      payload: { name: string; progress: number | null };
    }
  // LYN: Pattern
  | { action: "PatternAdded"; payload: { name: string; pattern: Pattern } }
  | { action: "PatternDeleted"; payload: { name: string } }
  | { action: "PatternEdited"; payload: { name: string; pattern: Pattern } }
  // LYN: Ticker
  | { action: "TickerPlaying" }
  | { action: "TickerPaused" }
  | { action: "TickerStopped" }
  | { action: "TickerTick"; payload: { tick: number; max: usize } }
  | { action: "TickerBpmUpdated"; payload: { bpm: number } }
  // LYN: Response
  | { action: "ResponseTickerBpm"; payload: { bpm: number } }
  | { action: "ResponseTickerPlaying"; payload: { playing: boolean } }
  | { action: "ResponseTickerTick"; payload: { tick: number; max: usize } }
  | { action: "ResponseProjectName"; payload: { name: string } }
  | { action: "ResponseCommAddr"; payload: { addr: string } }
  | { action: "ResponseCommStatus"; payload: { established: boolean } }
  | { action: "ResponseCtrlContext"; payload: { context: string | null } }
  | { action: "ResponseTrack"; payload: { name: string; track: Track } }
  | { action: "ResponseAllTracks"; payload: { tracks: Record<string, Track> } }
  | { action: "ResponsePattern"; payload: { name: string; pattern: Pattern } }
  | {
      action: "ResponseAllPatterns";
      payload: { patterns: Record<string, Pattern> };
    }
  | {
      action: "Notify";
      payload: { severity: Severity; summary: string; detail: string };
    };

export type Severity =
  | "success"
  | "info"
  | "warn"
  | "error"
  | "secondary"
  | "contrast";
