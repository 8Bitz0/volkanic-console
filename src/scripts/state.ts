import type { Runner } from "./runner";
import type { View } from "./view";

export interface AppState {
  runners: Map<string, Runner>;
  selectedInstance: [string, string] | null;
  view: View;
  settingsOpen: boolean;
  newInstanceModal: boolean;
  newRunnerModal: boolean;
  titleBarEnabled: boolean;
}
