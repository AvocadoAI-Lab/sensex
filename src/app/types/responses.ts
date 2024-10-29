import { AgentsResponse } from './agents';

export interface WazuhAffectedItem {
  id?: string;
  [key: string]: unknown;
}

export interface WazuhResponseData {
  affected_items: WazuhAffectedItem[];
  total_affected_items?: number;
  total_failed_items?: number;
  failed_items?: WazuhAffectedItem[];
}

export interface WazuhResponse {
  data: WazuhResponseData;
  error?: string;
}

export interface AllResponses {
  agents: AgentsResponse | null;
  clusterStatus: WazuhResponse | null;
  managerStatus: WazuhResponse | null;
  managerInfo: WazuhResponse | null;
  clusterLocalInfo: WazuhResponse | null;
  rules: WazuhResponse | null;
  decoders: WazuhResponse | null;
  managerLogs: WazuhResponse | null;
  managerStats: WazuhResponse | null;
  groups: WazuhResponse | null;
  groupFiles: WazuhResponse | null;
  groupAgents: WazuhResponse | null;
  syscollectorHardware: WazuhResponse | null;
  syscollectorOs: WazuhResponse | null;
  syscheckFiles: WazuhResponse | null;
  syscheckLastScan: WazuhResponse | null;
  activeResponseCommands: WazuhResponse | null;
  activeResponseRun: WazuhResponse | null;
  securityUsers: WazuhResponse | null;
  securityRoles: WazuhResponse | null;
  securityPolicies: WazuhResponse | null;
  mitreMetadata: WazuhResponse | null;
  mitreTechniques: WazuhResponse | null;
  mitreTactics: WazuhResponse | null;
  tasks: WazuhResponse | null;
  tasksStatus: WazuhResponse | null;
  tasksResult: WazuhResponse | null;
  scaChecks: WazuhResponse | null;
  scaResults: WazuhResponse | null;
  scaPolicies: WazuhResponse | null;
}
