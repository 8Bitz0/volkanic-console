import { invoke } from "@tauri-apps/api/core";

export interface Instance {
  name: string;
  type: InstanceType;
  status: InstanceStatus;
}

export type InstanceType = {
  volkanic: {
    source: VolkanicSource
  }
}

export type InstanceStatus =
  | "inactive"
  | "running"
  | { creating: number }
  | "deleting" 
  | "starting"
  | "stopping";

export type VolkanicSource =
  | { url: string }
  | { base64: string };

  export interface InstanceRequest {
    name: string;
    type: InstanceType;
  }

/**
 * Deletes a specified instance for a given runner.
 * @param runner - The identifier of the runner.
 * @param instance - The identifier of the instance to be deleted.
 * @returns A Promise that resolves when the instance is successfully deleted.
 */
export async function delInstance(runner: string, instance: string) {
  await invoke("del_instance", { runner, instance });
}

export async function newInstance(runner: string, instance: InstanceRequest) {
  await invoke("new_instance", { runner, instance });
}

export async function startInstance(runner: string, instance: string) {
  await invoke("start_instance", { runner, instance });
}

export async function stopInstance(runner: string, instance: string) {
  await invoke("stop_instance", { runner, instance });
}
