import { listen } from "@tauri-apps/api/event";

import type { Runner } from "./Runner";

export async function runnerListener(onRunnerChange: (runners: Map<string, Runner>) => void) {
  listen<Map<string, Runner>>("runner", (event) => {
    onRunnerChange(event.payload);
  });
}
