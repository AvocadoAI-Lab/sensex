type EndpointGroup = {
  title: string;
  endpoints: {
    title: string;
    key: keyof AllResponses;
  }[];
};

export const endpointGroups: EndpointGroup[] = [
  {
    title: "System Management",
    endpoints: [
      { title: 'Agents', key: 'agents' },
      { title: 'Cluster Status', key: 'clusterStatus' },
      { title: 'Manager Status', key: 'managerStatus' },
      { title: 'Manager Info', key: 'managerInfo' },
      { title: 'Cluster Local Info', key: 'clusterLocalInfo' },
    ]
  },
  {
    title: "Security Controls",
    endpoints: [
      { title: 'Rules', key: 'rules' },
      { title: 'Decoders', key: 'decoders' },
      { title: 'Manager Logs', key: 'managerLogs' },
      { title: 'Manager Stats', key: 'managerStats' },
    ]
  },
  {
    title: "Group Management",
    endpoints: [
      { title: 'Groups', key: 'groups' },
      { title: 'Group Files', key: 'groupFiles' },
      { title: 'Group Agents', key: 'groupAgents' },
    ]
  },
  {
    title: "System Collection",
    endpoints: [
      { title: 'Hardware Info', key: 'syscollectorHardware' },
      { title: 'OS Info', key: 'syscollectorOs' },
      { title: 'Files', key: 'syscheckFiles' },
      { title: 'Last Scan', key: 'syscheckLastScan' },
    ]
  },
  {
    title: "Response & Security",
    endpoints: [
      { title: 'Response Commands', key: 'activeResponseCommands' },
      { title: 'Response Run', key: 'activeResponseRun' },
      { title: 'Users', key: 'securityUsers' },
      { title: 'Roles', key: 'securityRoles' },
      { title: 'Policies', key: 'securityPolicies' },
    ]
  },
  {
    title: "MITRE Framework",
    endpoints: [
      { title: 'Metadata', key: 'mitreMetadata' },
      { title: 'Techniques', key: 'mitreTechniques' },
      { title: 'Tactics', key: 'mitreTactics' },
    ]
  },
  {
    title: "Tasks & Assessment",
    endpoints: [
      { title: 'Tasks', key: 'tasks' },
      { title: 'Task Status', key: 'tasksStatus' },
      { title: 'Task Result', key: 'tasksResult' },
      { title: 'SCA Checks', key: 'scaChecks' },
      { title: 'SCA Results', key: 'scaResults' },
      { title: 'SCA Policies', key: 'scaPolicies' },
    ]
  },
];
