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
