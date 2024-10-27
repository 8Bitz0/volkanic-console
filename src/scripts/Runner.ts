import { invoke } from "@tauri-apps/api/core";

export interface Runner {
  name: string;
  url: string;
  connected: boolean;
}

export async function newRunner(name: string, url: string) {
  await invoke("runner_new", { name, url });
}

export async function listRunners(): Promise<Map<string, Runner>> {
  return await invoke("runner_list")
}

export async function isValidUrl(url: string): Promise<boolean> {
  return await invoke("is_valid_url", { url });
}
