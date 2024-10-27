import { useState, useMemo } from 'react';
import { AgentsResponse, Agent } from '../types/agents';
import { format, parseISO } from 'date-fns';
import AgentDetailsPopover from './AgentDetailsPopover';

interface AgentsVisualizationProps {
  agents: AgentsResponse;
}

type SortField = 'id' | 'name' | 'status' | 'os.platform' | 'version' | 'lastKeepAlive';
type SortDirection = 'asc' | 'desc';

export default function AgentsVisualization({ agents }: AgentsVisualizationProps) {
  const [selectedAgent, setSelectedAgent] = useState<Agent | null>(null);
  const [sortField, setSortField] = useState<SortField>('id');
  const [sortDirection, setSortDirection] = useState<SortDirection>('asc');

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

  const formatDate = (dateString: string) => {
    try {
      return format(parseISO(dateString), 'yyyy-MM-dd HH:mm:ss');
    } catch {
      return 'Invalid date';
    }
  };

  const handleSort = (field: SortField) => {
    if (sortField === field) {
      setSortDirection(sortDirection === 'asc' ? 'desc' : 'asc');
    } else {
      setSortField(field);
      setSortDirection('asc');
    }
  };

  const sortedAgents = useMemo(() => {
    return [...agents.data.affected_items].sort((a, b) => {
      let compareResult = 0;
      
      switch (sortField) {
        case 'id':
        case 'name':
        case 'status':
        case 'version':
          compareResult = String(a[sortField]).localeCompare(String(b[sortField]));
          break;
        case 'os.platform':
          compareResult = String(a.os.platform).localeCompare(String(b.os.platform));
          break;
        case 'lastKeepAlive':
          compareResult = new Date(a.lastKeepAlive).getTime() - new Date(b.lastKeepAlive).getTime();
          break;
      }

      return sortDirection === 'asc' ? compareResult : -compareResult;
    });
  }, [agents.data.affected_items, sortField, sortDirection]);

  const SortIcon = ({ field }: { field: SortField }) => {
    if (sortField !== field) return null;
    return (
      <span className="ml-1">
        {sortDirection === 'asc' ? '↑' : '↓'}
      </span>
    );
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
      <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
        {/* OS Distribution */}
        <div className="bg-white p-4 rounded-lg shadow">
          <h3 className="text-lg font-semibold mb-4">OS Distribution</h3>
          <div className="space-y-2">
            {Object.entries(osDistribution).map(([os, count]) => (
              <div key={os} className="flex items-center">
                <div className="w-24 capitalize truncate" title={os}>{os}</div>
                <div className="flex-1">
                  <div className="h-4 bg-gray-200 rounded-full overflow-hidden">
                    <div 
                      className="h-full bg-blue-500"
                      style={{ width: `${(count / agents.data.total_affected_items) * 100}%` }}
                    />
                  </div>
                </div>
                <div className="w-12 text-right">{count}</div>
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
                <div className="w-24 truncate" title={version}>{version}</div>
                <div className="flex-1">
                  <div className="h-4 bg-gray-200 rounded-full overflow-hidden">
                    <div 
                      className="h-full bg-green-500"
                      style={{ width: `${(count / agents.data.total_affected_items) * 100}%` }}
                    />
                  </div>
                </div>
                <div className="w-12 text-right">{count}</div>
              </div>
            ))}
          </div>
        </div>
      </div>

      {/* Agents Table */}
      <div className="bg-white rounded-lg shadow overflow-hidden">
        <div className="overflow-x-auto">
          <table className="min-w-full divide-y divide-gray-200">
            <thead className="bg-gray-50">
              <tr>
                <th 
                  scope="col" 
                  className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider cursor-pointer"
                  onClick={() => handleSort('id')}
                >
                  ID <SortIcon field="id" />
                </th>
                <th 
                  scope="col" 
                  className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider cursor-pointer"
                  onClick={() => handleSort('name')}
                >
                  Name <SortIcon field="name" />
                </th>
                <th 
                  scope="col" 
                  className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider cursor-pointer"
                  onClick={() => handleSort('status')}
                >
                  Status <SortIcon field="status" />
                </th>
                <th 
                  scope="col" 
                  className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider cursor-pointer"
                  onClick={() => handleSort('os.platform')}
                >
                  OS <SortIcon field="os.platform" />
                </th>
                <th 
                  scope="col" 
                  className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider cursor-pointer"
                  onClick={() => handleSort('version')}
                >
                  Version <SortIcon field="version" />
                </th>
                <th 
                  scope="col" 
                  className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider cursor-pointer"
                  onClick={() => handleSort('lastKeepAlive')}
                >
                  Last Keep Alive <SortIcon field="lastKeepAlive" />
                </th>
              </tr>
            </thead>
            <tbody className="bg-white divide-y divide-gray-200">
              {sortedAgents.map((agent) => (
                <tr 
                  key={agent.id} 
                  className="hover:bg-gray-50 cursor-pointer"
                  onClick={() => setSelectedAgent(agent)}
                >
                  <td className="px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900">
                    {agent.id}
                  </td>
                  <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                    {agent.name}
                  </td>
                  <td className="px-6 py-4 whitespace-nowrap">
                    <span className={`inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium ${
                      statusColors[agent.status as keyof typeof statusColors] || 'bg-blue-100 text-blue-800'
                    }`}>
                      {agent.status}
                    </span>
                  </td>
                  <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                    {agent.os.platform}
                  </td>
                  <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                    {agent.version}
                  </td>
                  <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                    {formatDate(agent.lastKeepAlive)}
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      </div>

      {selectedAgent && (
        <AgentDetailsPopover 
          agent={selectedAgent} 
          onClose={() => setSelectedAgent(null)} 
        />
      )}
    </div>
  );
}
