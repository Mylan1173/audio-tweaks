export interface VideoStream {
  width: number | null;
  height: number | null;
  codecName: string | null;
  aspectRatio: string | null;
  pixelFormat: string | null;
  fieldOrder: string | null;
  profile: string | null;
  format: string | null;
}

export interface AudioStream {
  default: boolean;
  forced: boolean;
  title: string;
  language: string;
  isDeleted: boolean;
  isImported: boolean;
  path: string | null;
  bitRate: number | null;
  bitDepth: number | null;
  channels: number;
  newChannels: number;
  codecName: string;
  sampleRate: number | null;
  channelMap: Record<number | string, number | string | null>;
  channelVolumes: Record<number | string, number | string | null>;
}

export interface SubtitleStream {
  default: boolean;
  forced: boolean;
  title: string;
  language: string;
  isDeleted: boolean;
  isImported: boolean;
  path: string | null;
}

export interface PendingChanges {
  video: VideoStream | null;
  audio: AudioStream[] | null;
  subtitle: SubtitleStream[] | null;
}

export interface FFprobeAudioStream {
  codec_name?: string;
  codec_type: "audio";
  bit_rate?: number;
  bits_per_sample?: number;
  channels?: number;
  sample_rate?: number;
  tags?: {
    language?: string;
    title?: string;
  };
}

export interface TrackOption {
  code: number;
  name: string;
}

export interface Error {
  errorType: string;
}

export interface MediaDataClass {
  _video: VideoStream | null;
  _audio: AudioStream[];
  _subtitle: SubtitleStream[];

  errors: Error[];

  readonly isVideo: boolean;
  readonly isAudio: boolean;
  readonly isSubtitle: boolean;

  addError(errType: string): void;
  removeError(errType: string): void;
  isError(errType: string): void;

  getVideo(): VideoStream | null;
  setVideo<K extends keyof VideoStream>(
    property: K,
    value: VideoStream[K],
  ): void;

  getAudio(index: number): AudioStream | null;
  getAudioTracks(): TrackOption[];
  setAudio<K extends keyof AudioStream>(
    property: K,
    index: number,
    value: AudioStream[K] | "delete",
  ): void;
  addAudio(importPath: string, rawAudioStream?: FFprobeAudioStream): number;

  getSubtitle(index: number): SubtitleStream | null;
  getSubtitleTracks(): TrackOption[];
  setSubtitle<K extends keyof SubtitleStream>(
    property: K,
    index: number,
    value: SubtitleStream[K] | "delete",
  ): void;
  addSubtitle(importPath: string): number;

  exportProfile(): string;
  importProfile(jsonString: string): void;

  getTarget(category: "video" | "audio" | "subtitle", id: string): any;
  setTarget(
    category: "video" | "audio" | "subtitle",
    id: string,
    value: any,
  ): void;
}

export interface ExplorerNode {
  data_path: string;
  data_type: "File" | "Folder";
  children?: ExplorerNode[];
}

export interface EnvironmentState {
  dataPath?: string;
  isFile?: boolean;
}

export interface SelectedMediaState {
  mediaPath: string;
  mediaName: string;
  mediaType?: string;
}

export interface ModalState {
  showModal: (type: string, title: string, options?: any) => Promise<any>;
  handleSelect: (value: any) => void;
}

export interface QuickMenuCoords {
  top: number;
  left: number;
  height?: number;
}

export interface QuickMenuOptions {
  label: string;
  value: string;
  icon: string;
}

export interface QuickMenuState {
  isOpen: boolean;
  coords: QuickMenuCoords;
  options: QuickMenuOptions[];
  resolve: ((value: any) => void) | null;
}

import { MediaData, MediaDataComparer } from "./routes/utils/classes.svelte.js";

export interface AppState {
  explorer: ExplorerNode[];
  enviroment: EnvironmentState;
  selectedMedia: SelectedMediaState | undefined;
  sidebarWidth: number;
  modal: ModalState | null;
  data: MediaData;
  contentData: MediaDataComparer;
  quickMenu: QuickMenuState;
  activeDropdownId: string | number | null;
}
