import { AgentsResponse } from '../types/agents';

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

  const statusColors = {
    active: 'bg-green-100 text-green-800',
    disconnected: 'bg-red-100 text-red-800',
    pending: 'bg-yellow-100 text-yellow-800',
    never_connected: 'bg-gray-100 text-gray-800',
  };

  return (
    <div className="space-y-6">
      {/* Status Summary */}
      <div className="grid grid-cols-1 md:grid-cols-4 gap-4">
        {Object.entries(statusCounts).map(([status, count]) => (
          <div key={status} className={`p-4 rounded-lg ${statusColors[status as keyof typeof statusColors] || 'bg-blue-100 text-blue-800'}`}>
            <div className="text-2xl font-bold">{count}</div>
            <div className="text-sm capitalize">{status.replace('_', ' ')} Agents</div>
          </div>
        ))}
      </div>

      {/* OS Distribution */}
      <div className="bg-white p-4 rounded-lg shadow">
        <h3 className="text-lg font-semibold mb-4">OS Distribution</h3>
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

      {/* Version Distribution */}
      <div className="bg-white p-4 rounded-lg shadow">
        <h3 className="text-lg font-semibold mb-4">Version Distribution</h3>
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

      {/* Active Agents Table */}
      <div className="bg-white p-4 rounded-lg shadow overflow-x-auto">
        <h3 className="text-lg font-semibold mb-4">Active Agents</h3>
        <table className="min-w-full">
          <thead>
            <tr className="bg-gray-50">
              <th className="px-4 py-2 text-left">ID</th>
              <th className="px-4 py-2 text-left">Name</th>
              <th className="px-4 py-2 text-left">IP</th>
              <th className="px-4 py-2 text-left">OS</th>
              <th className="px-4 py-2 text-left">Version</th>
              <th className="px-4 py-2 text-left">Status</th>
            </tr>
          </thead>
          <tbody>
            {agents.data.affected_items.map((agent) => (
              <tr key={agent.id} className="border-t">
                <td className="px-4 py-2">{agent.id}</td>
                <td className="px-4 py-2">{agent.name}</td>
                <td className="px-4 py-2">{agent.ip}</td>
                <td className="px-4 py-2">{agent.os?.platform || 'N/A'} {agent.os?.version || ''}</td>
                <td className="px-4 py-2">{agent.version}</td>
                <td className="px-4 py-2">
                  <span className={`inline-block px-2 py-1 rounded-full text-xs ${statusColors[agent.status as keyof typeof statusColors] || 'bg-blue-100 text-blue-800'}`}>
                    {agent.status}
                  </span>
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </div>
  );
}
