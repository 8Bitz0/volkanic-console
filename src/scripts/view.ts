export type View =
  | { type: "home" }
  | { type: "instance-overview", runner: string, instance: string }
  | { type: "new-instance", runner: string, name: string };
