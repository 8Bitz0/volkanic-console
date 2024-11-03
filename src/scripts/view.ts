export type View =
  | { type: "home" }
  | { type: "instance-overview", runner: string, instance: string };
