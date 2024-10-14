export interface Instance {
  id: string;
  name: string;
}

export type InstanceType = {
  Volkanic: {
    source: VolkanicSource
  }
}

export type VolkanicSource =
  | { Url: [string] }
  | { Base64: [string] };
