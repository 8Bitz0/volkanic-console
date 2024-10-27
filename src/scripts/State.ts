import type { Instance } from "./Instance";
import type { Runner } from "./Runner";

export interface AppState {
  runners: Map<string, Runner>;
  instances: Instance[];
  selectedRunner?: string;
  selectedInstance?: string;
  settingsOpen: boolean;
  newInstanceModal: boolean;
  newRunnerModal: boolean;
  titleBarEnabled: boolean;
}
