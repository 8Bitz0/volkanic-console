import { invoke } from "@tauri-apps/api/core";

export interface Instance {
  name: string;
  type: InstanceType;
}

export type InstanceType = {
  volkanic: {
    source: VolkanicSource
  }
}

export type VolkanicSource =
  | { url: [string] }
  | { base64: [string] };

/**
 * Deletes a specified instance for a given runner.
 * @param runner - The identifier of the runner.
 * @param instance - The identifier of the instance to be deleted.
 * @returns A Promise that resolves when the instance is successfully deleted.
 */
export async function delInstance(runner: string, instance: string) {
  await invoke("del_instance", { runner, instance });
}
