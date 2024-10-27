'use client';
import { Agent } from '../types/agents';
import { format, parseISO } from 'date-fns';

interface AgentDetailsPopoverProps {
  agent: Agent;
  onClose: () => void;
}

export default function AgentDetailsPopover({ agent, onClose }: AgentDetailsPopoverProps) {
  const formatDate = (dateString: string) => {
    try {
      return format(parseISO(dateString), 'yyyy-MM-dd HH:mm:ss');
    } catch {
      return 'Invalid date';
    }
  };

  return (
    <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50" onClick={onClose}>
      <div className="bg-white rounded-lg shadow-xl max-w-2xl w-full mx-4 max-h-[90vh] overflow-y-auto" onClick={e => e.stopPropagation()}>
        <div className="p-6">
          <div className="flex justify-between items-start mb-4">
            <h2 className="text-2xl font-bold text-gray-900">Agent Details</h2>
            <button onClick={onClose} className="text-gray-400 hover:text-gray-500">
              <span className="sr-only">Close</span>
              <svg className="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" />
              </svg>
            </button>
          </div>

          <div className="grid grid-cols-2 gap-4">
            <div className="col-span-2">
              <div className="bg-gray-50 rounded-lg p-4">
                <h3 className="text-lg font-semibold text-gray-900 mb-2">{agent.name}</h3>
                <div className="grid grid-cols-2 gap-2 text-sm">
                  <div>ID: {agent.id}</div>
                  <div>Version: {agent.version}</div>
                </div>
              </div>
            </div>

            <div className="space-y-4">
              <div>
                <h4 className="text-sm font-medium text-gray-500">Status Information</h4>
                <div className="mt-2 text-sm">
                  <div>Status: {agent.status}</div>
                  <div>Last Keep Alive: {formatDate(agent.lastKeepAlive)}</div>
                  {agent.disconnection_time && (
                    <div>Disconnection: {formatDate(agent.disconnection_time)}</div>
                  )}
                </div>
              </div>

              <div>
                <h4 className="text-sm font-medium text-gray-500">Network Information</h4>
                <div className="mt-2 text-sm">
                  <div>IP: {agent.ip}</div>
                  <div>Register IP: {agent.registerIP}</div>
                </div>
              </div>

              <div>
                <h4 className="text-sm font-medium text-gray-500">Groups</h4>
                <div className="mt-2">
                  {agent.group?.map(group => (
                    <span key={group} className="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-blue-100 text-blue-800 mr-2 mb-2">
                      {group}
                    </span>
                  ))}
                </div>
                <div className="text-xs text-gray-500 mt-1">
                  Configuration Status: {agent.group_config_status}
                </div>
              </div>
            </div>

            <div className="space-y-4">
              <div>
                <h4 className="text-sm font-medium text-gray-500">Operating System</h4>
                <div className="mt-2 text-sm">
                  <div>Name: {agent.os.name}</div>
                  <div>Version: {agent.os.version}</div>
                  <div>Platform: {agent.os.platform}</div>
                  <div>Architecture: {agent.os.arch}</div>
                  {agent.os.codename && <div>Codename: {agent.os.codename}</div>}
                </div>
              </div>

              <div>
                <h4 className="text-sm font-medium text-gray-500">Management</h4>
                <div className="mt-2 text-sm">
                  <div>Manager: {agent.manager}</div>
                  <div>Node: {agent.node_name}</div>
                </div>
              </div>

              <div>
                <h4 className="text-sm font-medium text-gray-500">Registration</h4>
                <div className="mt-2 text-sm">
                  <div>Date: {formatDate(agent.dateAdd)}</div>
                  {agent.configSum && <div>Config Sum: {agent.configSum}</div>}
                  {agent.mergedSum && <div>Merged Sum: {agent.mergedSum}</div>}
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
