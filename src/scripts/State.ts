import type { Instance } from "./Instance";
import type { Runner } from "./Runner";

export interface AppState {
  runners: Map<string, Runner>;
  selectedInstance: [string, string] | null;
  settingsOpen: boolean;
  newInstanceModal: boolean;
  newRunnerModal: boolean;
  titleBarEnabled: boolean;
}
