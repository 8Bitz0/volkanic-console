export type View =
  | { type: "home" }
  | { type: "instance", runner: string, instance: string };
