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

  // Calculate statistics
  const stats = useMemo(() => {
    const activeCount = agents.data.affected_items.filter(a => a.status === 'active').length;
    const disconnectedCount = agents.data.affected_items.filter(a => a.status === 'disconnected').length;
    
    const osDistribution = agents.data.affected_items.reduce((acc, agent) => {
      const platform = agent.os?.platform || 'unknown';
      acc[platform] = (acc[platform] || 0) + 1;
      return acc;
    }, {} as Record<string, number>);

    const versionDistribution = agents.data.affected_items.reduce((acc, agent) => {
      const version = agent.version || 'unknown';
      acc[version] = (acc[version] || 0) + 1;
      return acc;
    }, {} as Record<string, number>);

    return {
      total: agents.data.total_affected_items,
      active: activeCount,
      disconnected: disconnectedCount,
      osDistribution,
      versionDistribution
    };
  }, [agents]);

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

  return (
    <div className="space-y-8">
      {/* Statistics Cards */}
      <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
        <div className="bg-white rounded-xl shadow-sm p-6">
          <div className="flex items-center justify-between">
            <h3 className="text-gray-500 text-sm font-medium">Total Agents</h3>
            <span className="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-gray-100 text-gray-800">
              Total
            </span>
          </div>
          <p className="mt-2 text-3xl font-bold text-gray-900">{stats.total}</p>
        </div>

        <div className="bg-white rounded-xl shadow-sm p-6">
          <div className="flex items-center justify-between">
            <h3 className="text-gray-500 text-sm font-medium">Active Agents</h3>
            <span className="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-green-100 text-green-800">
              Online
            </span>
          </div>
          <p className="mt-2 text-3xl font-bold text-green-600">{stats.active}</p>
          <p className="mt-1 text-sm text-gray-500">{((stats.active / stats.total) * 100).toFixed(1)}% of total</p>
        </div>

        <div className="bg-white rounded-xl shadow-sm p-6">
          <div className="flex items-center justify-between">
            <h3 className="text-gray-500 text-sm font-medium">Disconnected Agents</h3>
            <span className="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-red-100 text-red-800">
              Offline
            </span>
          </div>
          <p className="mt-2 text-3xl font-bold text-red-600">{stats.disconnected}</p>
          <p className="mt-1 text-sm text-gray-500">{((stats.disconnected / stats.total) * 100).toFixed(1)}% of total</p>
        </div>
      </div>

      {/* Distribution Charts */}
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        {/* OS Distribution */}
        <div className="bg-white rounded-xl shadow-sm p-6">
          <h3 className="text-lg font-medium text-gray-900 mb-4">OS Distribution</h3>
          <div className="space-y-4">
            {Object.entries(stats.osDistribution).map(([os, count]) => (
              <div key={os}>
                <div className="flex justify-between text-sm text-gray-600 mb-1">
                  <span className="font-medium">{os}</span>
                  <span>{count} ({((count / stats.total) * 100).toFixed(1)}%)</span>
                </div>
                <div className="w-full bg-gray-200 rounded-full h-2">
                  <div
                    className="bg-blue-600 h-2 rounded-full"
                    style={{ width: `${(count / stats.total) * 100}%` }}
                  />
                </div>
              </div>
            ))}
          </div>
        </div>

        {/* Version Distribution */}
        <div className="bg-white rounded-xl shadow-sm p-6">
          <h3 className="text-lg font-medium text-gray-900 mb-4">Version Distribution</h3>
          <div className="space-y-4">
            {Object.entries(stats.versionDistribution).map(([version, count]) => (
              <div key={version}>
                <div className="flex justify-between text-sm text-gray-600 mb-1">
                  <span className="font-medium">{version}</span>
                  <span>{count} ({((count / stats.total) * 100).toFixed(1)}%)</span>
                </div>
                <div className="w-full bg-gray-200 rounded-full h-2">
                  <div
                    className="bg-green-600 h-2 rounded-full"
                    style={{ width: `${(count / stats.total) * 100}%` }}
                  />
                </div>
              </div>
            ))}
          </div>
        </div>
      </div>

      {/* Agents Table */}
      <div className="bg-white rounded-xl shadow-sm overflow-hidden">
        <div className="overflow-x-auto">
          <table className="min-w-full divide-y divide-gray-200">
            <thead>
              <tr className="bg-gray-50">
                {[
                  { key: 'id' as SortField, label: 'ID' },
                  { key: 'name' as SortField, label: 'Name' },
                  { key: 'status' as SortField, label: 'Status' },
                  { key: 'os.platform' as SortField, label: 'OS' },
                  { key: 'version' as SortField, label: 'Version' },
                  { key: 'lastKeepAlive' as SortField, label: 'Last Keep Alive' }
                ].map(({ key, label }) => (
                  <th
                    key={key}
                    onClick={() => handleSort(key)}
                    className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider cursor-pointer hover:bg-gray-100"
                  >
                    <div className="flex items-center space-x-1">
                      <span>{label}</span>
                      {sortField === key && (
                        <span className="text-gray-400">
                          {sortDirection === 'asc' ? '↑' : '↓'}
                        </span>
                      )}
                    </div>
                  </th>
                ))}
              </tr>
            </thead>
            <tbody className="divide-y divide-gray-200">
              {sortedAgents.map((agent) => (
                <tr
                  key={agent.id}
                  onClick={() => setSelectedAgent(agent)}
                  className="hover:bg-gray-50 cursor-pointer transition-colors"
                >
                  <td className="px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900">
                    {agent.id}
                  </td>
                  <td className="px-6 py-4 text-sm text-gray-900 max-w-xs truncate">
                    {agent.name}
                  </td>
                  <td className="px-6 py-4 whitespace-nowrap">
                    <span className={`inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium ${
                      agent.status === 'active'
                        ? 'bg-green-100 text-green-800'
                        : 'bg-red-100 text-red-800'
                    }`}>
                      {agent.status}
                    </span>
                  </td>
                  <td className="px-6 py-4 text-sm text-gray-500 max-w-xs truncate">
                    {agent.os.platform}
                  </td>
                  <td className="px-6 py-4 text-sm text-gray-500 max-w-xs truncate">
                    {agent.version}
                  </td>
                  <td className="px-6 py-4 text-sm text-gray-500 whitespace-nowrap">
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
