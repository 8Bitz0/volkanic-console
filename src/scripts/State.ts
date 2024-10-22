import type { Instance } from "./Instance";
import type { Runner } from "./Runner";

export interface AppState {
  runners: Runner[];
  instances: Instance[];
  selectedRunner?: string;
  selectedInstance?: string;
  settingsOpen: boolean;
  newInstanceModal: boolean;
  titleBarEnabled: boolean;
}
