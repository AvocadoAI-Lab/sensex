export interface Root {
  took: number;
  timed_out: boolean;
  _shards: Shards;
  hits: Hits;
}

export interface Shards {
  total: number;
  successful: number;
  skipped: number;
  failed: number;
}

export interface Hits {
  total: Total;
  max_score: number | null;
  hits: Hit[];
}

export interface Total {
  value: number;
  relation: string;
}

export interface Hit {
  _index: string;
  _id: string;
  _score: number | null;
  _source: Source;
  sort: number[];
}

export interface Source {
  agent: Agent;
  manager: Manager;
  rule: Rule;
  location: string;
  decoder: Decoder;
  timestamp: string;
  data?: Data;
}

export interface Agent {
  name: string;
  id: string;
}

export interface Manager {
  name: string;
}

export interface Rule {
  level: number;
  description: string;
  groups: string[];
  id: string;
}

export interface Decoder {
  name: string;
}

export interface Data {
  file?: string;
  title?: string;
  sca?: Sca;
}

export interface Sca {
  scan_id: string;
  check?: Check;
  type: string;
  policy: string;
  score?: string;
  total_checks?: string;
  file?: string;
  policy_id?: string;
  invalid?: string;
  description?: string;
  passed?: string;
  failed?: string;
}

export interface Check {
  result: string;
  remediation: string;
  previous_result: string;
  compliance: Compliance;
  description: string;
  id: string;
  title: string;
  rationale: string;
  command: string[];
}

export interface Compliance {
  cis_csc_v8: string;
  soc_2: string;
  pci_dss_v3: PciDssV3;
  'nist_sp_800-53': string;
  mitre_techniques: string;
  cis: string;
  cmmc_v2: CmmcV2;
  cis_csc_v7: string;
}

export interface PciDssV3 {
  '2': {
    '1': string;
  };
}

export interface CmmcV2 {
  '0': string;
}
