import { AgentsResponse } from '../types/agents';
import { format, parseISO } from 'date-fns';

interface AgentsVisualizationProps {
  agents: AgentsResponse;
}

export default function AgentsVisualization({ agents }: AgentsVisualizationProps) {
  // Calculate status counts
  const statusCounts = agents.data.affected_items.reduce((acc, agent) => {
    acc[agent.status] = (acc[agent.status] || 0) + 1;
    return acc;
  }, {} as Record<string, number>);

  // Calculate OS distribution
  const osDistribution = agents.data.affected_items.reduce((acc, agent) => {
    const platform = agent.os?.platform || 'unknown';
    acc[platform] = (acc[platform] || 0) + 1;
    return acc;
  }, {} as Record<string, number>);

  // Calculate version distribution
  const versionDistribution = agents.data.affected_items.reduce((acc, agent) => {
    const version = agent.version || 'unknown';
    acc[version] = (acc[version] || 0) + 1;
    return acc;
  }, {} as Record<string, number>);

  // Calculate group distribution
  const groupDistribution = agents.data.affected_items.reduce((acc, agent) => {
    const groups = agent.group || ['no group'];
    groups.forEach(group => {
      acc[group] = (acc[group] || 0) + 1;
    });
    return acc;
  }, {} as Record<string, number>);

  // Calculate OS version distribution
  const osVersionDistribution = agents.data.affected_items.reduce((acc, agent) => {
    const osVersion = `${agent.os.name} ${agent.os.version}`;
    acc[osVersion] = (acc[osVersion] || 0) + 1;
    return acc;
  }, {} as Record<string, number>);

  const statusColors = {
    active: 'bg-green-100 text-green-800',
    disconnected: 'bg-red-100 text-red-800',
    pending: 'bg-yellow-100 text-yellow-800',
    never_connected: 'bg-gray-100 text-gray-800',
  };

  const formatDate = (dateString: string) => {
    try {
      return format(parseISO(dateString), 'yyyy-MM-dd HH:mm:ss');
    } catch {
      return 'Invalid date';
    }
  };

  return (
    <div className="space-y-6">
      {/* Summary Cards */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-5 gap-4">
        <div className="bg-white p-4 rounded-lg shadow">
          <h3 className="text-sm font-medium text-gray-500">Total Agents</h3>
          <p className="text-2xl font-bold text-gray-900">{agents.data.total_affected_items}</p>
        </div>
        {Object.entries(statusCounts).map(([status, count]) => (
          <div key={status} className={`p-4 rounded-lg ${statusColors[status as keyof typeof statusColors] || 'bg-blue-100 text-blue-800'}`}>
            <h3 className="text-sm font-medium capitalize">{status.replace('_', ' ')}</h3>
            <p className="text-2xl font-bold">{count}</p>
          </div>
        ))}
      </div>

      {/* Distribution Charts */}
      <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
        {/* OS Platform Distribution */}
        <div className="bg-white p-4 rounded-lg shadow">
          <h3 className="text-lg font-semibold mb-4">OS Platform Distribution</h3>
          <div className="space-y-2">
            {Object.entries(osDistribution).map(([os, count]) => (
              <div key={os} className="flex items-center">
                <div className="w-32 capitalize">{os}</div>
                <div className="flex-1">
                  <div className="h-4 bg-gray-200 rounded-full overflow-hidden">
                    <div 
                      className="h-full bg-blue-500"
                      style={{ width: `${(count / agents.data.total_affected_items) * 100}%` }}
                    />
                  </div>
                </div>
                <div className="w-16 text-right">{count}</div>
              </div>
            ))}
          </div>
        </div>

        {/* OS Version Distribution */}
        <div className="bg-white p-4 rounded-lg shadow">
          <h3 className="text-lg font-semibold mb-4">OS Version Distribution</h3>
          <div className="space-y-2 max-h-80 overflow-y-auto">
            {Object.entries(osVersionDistribution).map(([version, count]) => (
              <div key={version} className="flex items-center">
                <div className="w-48 truncate" title={version}>{version}</div>
                <div className="flex-1">
                  <div className="h-4 bg-gray-200 rounded-full overflow-hidden">
                    <div 
                      className="h-full bg-purple-500"
                      style={{ width: `${(count / agents.data.total_affected_items) * 100}%` }}
                    />
                  </div>
                </div>
                <div className="w-16 text-right">{count}</div>
              </div>
            ))}
          </div>
        </div>

        {/* Version Distribution */}
        <div className="bg-white p-4 rounded-lg shadow">
          <h3 className="text-lg font-semibold mb-4">Agent Version Distribution</h3>
          <div className="space-y-2">
            {Object.entries(versionDistribution).map(([version, count]) => (
              <div key={version} className="flex items-center">
                <div className="w-32">{version}</div>
                <div className="flex-1">
                  <div className="h-4 bg-gray-200 rounded-full overflow-hidden">
                    <div 
                      className="h-full bg-green-500"
                      style={{ width: `${(count / agents.data.total_affected_items) * 100}%` }}
                    />
                  </div>
                </div>
                <div className="w-16 text-right">{count}</div>
              </div>
            ))}
          </div>
        </div>

        {/* Group Distribution */}
        <div className="bg-white p-4 rounded-lg shadow">
          <h3 className="text-lg font-semibold mb-4">Group Distribution</h3>
          <div className="space-y-2 max-h-80 overflow-y-auto">
            {Object.entries(groupDistribution).map(([group, count]) => (
              <div key={group} className="flex items-center">
                <div className="w-32 truncate" title={group}>{group}</div>
                <div className="flex-1">
                  <div className="h-4 bg-gray-200 rounded-full overflow-hidden">
                    <div 
                      className="h-full bg-yellow-500"
                      style={{ width: `${(count / agents.data.total_affected_items) * 100}%` }}
                    />
                  </div>
                </div>
                <div className="w-16 text-right">{count}</div>
              </div>
            ))}
          </div>
        </div>
      </div>

      {/* Detailed Agents Table */}
      <div className="bg-white p-4 rounded-lg shadow overflow-x-auto">
        <h3 className="text-lg font-semibold mb-4">Agents Details</h3>
        <table className="min-w-full divide-y divide-gray-200">
          <thead className="bg-gray-50">
            <tr>
              <th scope="col" className="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">ID</th>
              <th scope="col" className="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Name</th>
              <th scope="col" className="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">IP</th>
              <th scope="col" className="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Status</th>
              <th scope="col" className="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Groups</th>
              <th scope="col" className="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">OS</th>
              <th scope="col" className="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Version</th>
              <th scope="col" className="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Node</th>
              <th scope="col" className="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Last Keep Alive</th>
              <th scope="col" className="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Registration</th>
            </tr>
          </thead>
          <tbody className="bg-white divide-y divide-gray-200">
            {agents.data.affected_items.map((agent) => (
              <tr key={agent.id} className="hover:bg-gray-50">
                <td className="px-4 py-2 whitespace-nowrap text-sm text-gray-900">{agent.id}</td>
                <td className="px-4 py-2 whitespace-nowrap text-sm text-gray-900">{agent.name}</td>
                <td className="px-4 py-2 whitespace-nowrap text-sm text-gray-500">
                  <div>{agent.ip}</div>
                  <div className="text-xs text-gray-400">Register: {agent.registerIP}</div>
                </td>
                <td className="px-4 py-2 whitespace-nowrap">
                  <span className={`inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium ${
                    statusColors[agent.status as keyof typeof statusColors] || 'bg-blue-100 text-blue-800'
                  }`}>
                    {agent.status}
                  </span>
                  {agent.disconnection_time && (
                    <div className="text-xs text-gray-400 mt-1">
                      Disconnected: {formatDate(agent.disconnection_time)}
                    </div>
                  )}
                </td>
                <td className="px-4 py-2 whitespace-nowrap text-sm text-gray-500">
                  <div className="space-y-1">
                    {agent.group?.map(group => (
                      <span key={group} className="inline-flex items-center px-2 py-0.5 rounded text-xs font-medium bg-gray-100 text-gray-800 mr-1">
                        {group}
                      </span>
                    ))}
                  </div>
                  <div className="text-xs text-gray-400 mt-1">
                    Config: {agent.group_config_status}
                  </div>
                </td>
                <td className="px-4 py-2 whitespace-nowrap text-sm text-gray-500">
                  <div>{agent.os.name} {agent.os.version}</div>
                  <div className="text-xs text-gray-400">{agent.os.platform} ({agent.os.arch})</div>
                  {agent.os.codename && (
                    <div className="text-xs text-gray-400">{agent.os.codename}</div>
                  )}
                </td>
                <td className="px-4 py-2 whitespace-nowrap text-sm text-gray-500">{agent.version}</td>
                <td className="px-4 py-2 whitespace-nowrap text-sm text-gray-500">
                  <div>{agent.node_name}</div>
                  <div className="text-xs text-gray-400">{agent.manager}</div>
                </td>
                <td className="px-4 py-2 whitespace-nowrap text-sm text-gray-500">
                  {formatDate(agent.lastKeepAlive)}
                </td>
                <td className="px-4 py-2 whitespace-nowrap text-sm text-gray-500">
                  {formatDate(agent.dateAdd)}
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </div>
  );
}
