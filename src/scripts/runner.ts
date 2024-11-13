import { invoke } from "@tauri-apps/api/core";

import type { Instance } from "./instance";

export interface Runner {
  name: string;
  url: string;
  connected: boolean;
  instances: Map<string, Instance>
}

/**
 * Creates a new runner with the specified name and URL.
 * 
 * @param name - The name of the runner to create
 * @param url - The URL associated with the runner
 * @returns Promise that resolves when the runner is created
 */
export async function newRunner(name: string, url: string) {
  await invoke("runner_new", { name, url });
}

/**
 * Retrieves a list of all runners in the system.
 * 
 * This function makes a call to the backend to get all registered runners
 * and their instances. It then converts the returned object structure into
 * a nested Map structure for easier manipulation.
 * 
 * @returns Promise<Map<string, Runner>> A Map where keys are runner IDs and values are Runner objects.
 *                                      Each Runner object contains an 'instances' property which is also
 *                                      converted to a Map structure.
 */
export async function listRunners(): Promise<Map<string, Runner>> {
  const runnersObj = await invoke<Record<string, Runner>>("runner_list");
  // Convert the returned object to a Map
  let runners = new Map(Object.entries(runnersObj))
  for (let runner of runners.values()) {
    runner.instances = new Map(Object.entries(runner.instances));
  }

  return runners;
}

/**
 * Checks if the provided URL is valid.
 * @param url - The URL string to validate
 * @returns A Promise that resolves to a boolean indicating whether the URL is valid
 */
export async function isValidUrl(url: string): Promise<boolean> {
  return await invoke("is_valid_url", { url });
}
