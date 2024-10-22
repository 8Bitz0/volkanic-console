import { invoke } from "@tauri-apps/api/core";

import { type AppState } from "./State";

export interface Runner {
  id: String;
  name: String;
}

export async function newRunner(state: AppState, url: string) {
  let r = invoke("runner_info", { url });

  alert(r);
}
