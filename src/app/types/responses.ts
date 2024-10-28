import { AgentsResponse } from './agents';

export type WazuhResponse = {
  data?: unknown;
  error?: string;
};

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
