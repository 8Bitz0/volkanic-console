import type { Runner } from "./runner";

export interface AppState {
  runners: Map<string, Runner>;
  selectedInstance: [string, string] | null;
  settingsOpen: boolean;
  newInstanceModal: boolean;
  newRunnerModal: boolean;
  titleBarEnabled: boolean;
}
