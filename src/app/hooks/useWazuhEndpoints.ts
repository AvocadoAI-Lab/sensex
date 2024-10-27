import { useState } from 'react';
import { WazuhResponse, AllResponses } from '../types/responses';

// Map endpoint keys to their actual API paths
const endpointPaths: Record<keyof AllResponses, string> = {
  agents: 'agents',
  clusterStatus: 'cluster/status',
  managerStatus: 'manager/status',
  managerInfo: 'manager/info',
  clusterLocalInfo: 'cluster/local/info',
  rules: 'rules',
  decoders: 'decoders',
  managerLogs: 'manager/logs',
  managerStats: 'manager/stats',
  groups: 'groups',
  groupFiles: 'groups/files',
  groupAgents: 'groups/agents',
  syscollectorHardware: 'syscollector/hardware',
  syscollectorOs: 'syscollector/os',
  syscheckFiles: 'syscheck/files',
  syscheckLastScan: 'syscheck/last_scan',
  activeResponseCommands: 'active-response/commands',
  activeResponseRun: 'active-response/run',
  securityUsers: 'security/users',
  securityRoles: 'security/roles',
  securityPolicies: 'security/policies',
  mitreMetadata: 'mitre/metadata',
  mitreTechniques: 'mitre/techniques',
  mitreTactics: 'mitre/tactics',
  tasks: 'tasks',
  tasksStatus: 'tasks/status',
  tasksResult: 'tasks/result',
  scaChecks: 'sca/checks',
  scaResults: 'sca/results',
  scaPolicies: 'sca/policies'
};

export const useWazuhEndpoints = () => {
  const [responses, setResponses] = useState<AllResponses>({
    agents: null,
    clusterStatus: null,
    managerStatus: null,
    managerInfo: null,
    clusterLocalInfo: null,
    rules: null,
    decoders: null,
    managerLogs: null,
    managerStats: null,
    groups: null,
    groupFiles: null,
    groupAgents: null,
    syscollectorHardware: null,
    syscollectorOs: null,
    syscheckFiles: null,
    syscheckLastScan: null,
    activeResponseCommands: null,
    activeResponseRun: null,
    securityUsers: null,
    securityRoles: null,
    securityPolicies: null,
    mitreMetadata: null,
    mitreTechniques: null,
    mitreTactics: null,
    tasks: null,
    tasksStatus: null,
    tasksResult: null,
    scaChecks: null,
    scaResults: null,
    scaPolicies: null,
  });

  const fetchEndpoint = async (
    key: keyof AllResponses,
    authToken: string,
    endpoint: string
  ) => {
    try {
      const response = await fetch(`/api/${endpointPaths[key]}`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          endpoint,
          token: authToken
        }),
      });
      const data = await response.json();
      return { key, data };
    } catch (error) {
      return { key, data: { error: (error as Error).message } };
    }
  };

  const fetchAllEndpoints = async (authToken: string, endpoint: string) => {
    // Create array of all endpoint keys
    const endpointKeys = Object.keys(endpointPaths) as Array<keyof AllResponses>;

    // Fetch all endpoints in parallel
    const results = await Promise.all(
      endpointKeys.map(key => fetchEndpoint(key, authToken, endpoint))
    );

    // Update state with all results at once
    setResponses(prev => {
      const newResponses = { ...prev };
      results.forEach(result => {
        newResponses[result.key] = result.data;
      });
      return newResponses;
    });
  };

  return {
    responses,
    fetchAllEndpoints
  };
};
