import { listen } from "@tauri-apps/api/event";

import type { Runner } from "./runner";

/**
 * Sets up a listener for runner changes and transforms the received payload into a Map structure.
 * 
 * @param onRunnerChange - Callback function that receives a Map of runners indexed by string keys.
 *                        The callback is invoked whenever there are changes to the runners.
 * 
 * @remarks
 * This function transforms the received runner payload by:
 * 1. Converting the top-level object into a Map of runners
 * 2. Converting each runner's instances object into a Map
 * 
 * @returns A Promise that resolves when the listener is set up
 */
export async function runnerListener(onRunnerChange: (runners: Map<string, Runner>) => void) {
  listen<Record<string, Runner>>("runner", (event) => {
    let runnersMap = new Map(Object.entries(event.payload));

    for (let runner of runnersMap.values()) {
      runner.instances = new Map(Object.entries(runner.instances));
    }

    onRunnerChange(runnersMap);
  });
}
