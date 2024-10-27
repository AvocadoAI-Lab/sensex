import { useState } from 'react';
import { WazuhResponse, AllResponses } from '../types/responses';

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
    path: string,
    authToken: string,
    endpoint: string,
    responseKey: keyof AllResponses
  ) => {
    try {
      const response = await fetch(`/api/${path}`, {
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
      setResponses(prev => ({
        ...prev,
        [responseKey]: data
      }));
      return data;
    } catch (error) {
      const errorResponse = { error: (error as Error).message };
      setResponses(prev => ({
        ...prev,
        [responseKey]: errorResponse
      }));
      return errorResponse;
    }
  };

  const fetchAllEndpoints = async (authToken: string, endpoint: string) => {
    // Existing endpoints
    await fetchEndpoint('agents', authToken, endpoint, 'agents');
    await fetchEndpoint('cluster/status', authToken, endpoint, 'clusterStatus');
    await fetchEndpoint('manager/status', authToken, endpoint, 'managerStatus');
    await fetchEndpoint('manager/info', authToken, endpoint, 'managerInfo');
    await fetchEndpoint('cluster/local/info', authToken, endpoint, 'clusterLocalInfo');
    await fetchEndpoint('rules', authToken, endpoint, 'rules');
    await fetchEndpoint('decoders', authToken, endpoint, 'decoders');
    await fetchEndpoint('manager/logs', authToken, endpoint, 'managerLogs');
    await fetchEndpoint('manager/stats', authToken, endpoint, 'managerStats');

    // New endpoints
    await fetchEndpoint('groups', authToken, endpoint, 'groups');
    await fetchEndpoint('groups/files', authToken, endpoint, 'groupFiles');
    await fetchEndpoint('groups/agents', authToken, endpoint, 'groupAgents');
    await fetchEndpoint('syscollector/hardware', authToken, endpoint, 'syscollectorHardware');
    await fetchEndpoint('syscollector/os', authToken, endpoint, 'syscollectorOs');
    await fetchEndpoint('syscheck/files', authToken, endpoint, 'syscheckFiles');
    await fetchEndpoint('syscheck/last_scan', authToken, endpoint, 'syscheckLastScan');
    await fetchEndpoint('active-response/commands', authToken, endpoint, 'activeResponseCommands');
    await fetchEndpoint('active-response/run', authToken, endpoint, 'activeResponseRun');
    await fetchEndpoint('security/users', authToken, endpoint, 'securityUsers');
    await fetchEndpoint('security/roles', authToken, endpoint, 'securityRoles');
    await fetchEndpoint('security/policies', authToken, endpoint, 'securityPolicies');
    await fetchEndpoint('mitre/metadata', authToken, endpoint, 'mitreMetadata');
    await fetchEndpoint('mitre/techniques', authToken, endpoint, 'mitreTechniques');
    await fetchEndpoint('mitre/tactics', authToken, endpoint, 'mitreTactics');
    await fetchEndpoint('tasks', authToken, endpoint, 'tasks');
    await fetchEndpoint('tasks/status', authToken, endpoint, 'tasksStatus');
    await fetchEndpoint('tasks/result', authToken, endpoint, 'tasksResult');
    await fetchEndpoint('sca/checks', authToken, endpoint, 'scaChecks');
    await fetchEndpoint('sca/results', authToken, endpoint, 'scaResults');
    await fetchEndpoint('sca/policies', authToken, endpoint, 'scaPolicies');
  };

  return {
    responses,
    fetchAllEndpoints
  };
};
