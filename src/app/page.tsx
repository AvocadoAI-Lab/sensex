'use client';
import { useState } from 'react';
import WazuhAuthForm from './components/WazuhAuthForm';
import CollapsibleSection from './components/CollapsibleSection';
import { AgentsResponse } from './types/agents';

type WazuhResponse = {
  data?: unknown;
  error?: string;
};

export default function Home() {
  const [token, setToken] = useState<string | null>(null);
  const [agents, setAgents] = useState<AgentsResponse | null>(null);
  const [error, setError] = useState<string | null>(null);
  
  // New state variables for each endpoint
  const [clusterStatus, setClusterStatus] = useState<WazuhResponse | null>(null);
  const [managerStatus, setManagerStatus] = useState<WazuhResponse | null>(null);
  const [managerInfo, setManagerInfo] = useState<WazuhResponse | null>(null);
  const [clusterLocalInfo, setClusterLocalInfo] = useState<WazuhResponse | null>(null);
  const [rules, setRules] = useState<WazuhResponse | null>(null);
  const [decoders, setDecoders] = useState<WazuhResponse | null>(null);
  const [managerLogs, setManagerLogs] = useState<WazuhResponse | null>(null);
  const [managerStats, setManagerStats] = useState<WazuhResponse | null>(null);

  const handleSubmit = async (formData: {
    endpoint: string;
    username: string;
    password: string;
  }) => {
    try {
      const response = await fetch('/api/auth', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(formData),
      });
      
      const data = await response.json();
      if (data.token) {
        setToken(data.token);
        setError(null);
        // Fetch all data after authentication
        fetchAgents(data.token, formData.endpoint);
        fetchClusterStatus(data.token, formData.endpoint);
        fetchManagerStatus(data.token, formData.endpoint);
        fetchManagerInfo(data.token, formData.endpoint);
        fetchClusterLocalInfo(data.token, formData.endpoint);
        fetchRules(data.token, formData.endpoint);
        fetchDecoders(data.token, formData.endpoint);
        fetchManagerLogs(data.token, formData.endpoint);
        fetchManagerStats(data.token, formData.endpoint);
      } else {
        setError(data.error || 'Authentication failed');
      }
    } catch (error) {
      setError((error as Error).message);
    }
  };

  const fetchAgents = async (authToken: string, endpoint: string) => {
    try {
      const response = await fetch('/api/agents', {
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
      if (data.error) {
        setError(data.error);
      } else {
        setAgents(data);
      }
    } catch (error) {
      setError((error as Error).message);
    }
  };

  const fetchClusterStatus = async (authToken: string, endpoint: string) => {
    try {
      const response = await fetch('/api/cluster/status', {
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
      setClusterStatus(data);
    } catch (error) {
      setClusterStatus({ error: (error as Error).message });
    }
  };

  const fetchManagerStatus = async (authToken: string, endpoint: string) => {
    try {
      const response = await fetch('/api/manager/status', {
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
      setManagerStatus(data);
    } catch (error) {
      setManagerStatus({ error: (error as Error).message });
    }
  };

  const fetchManagerInfo = async (authToken: string, endpoint: string) => {
    try {
      const response = await fetch('/api/manager/info', {
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
      setManagerInfo(data);
    } catch (error) {
      setManagerInfo({ error: (error as Error).message });
    }
  };

  const fetchClusterLocalInfo = async (authToken: string, endpoint: string) => {
    try {
      const response = await fetch('/api/cluster/local/info', {
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
      setClusterLocalInfo(data);
    } catch (error) {
      setClusterLocalInfo({ error: (error as Error).message });
    }
  };

  const fetchRules = async (authToken: string, endpoint: string) => {
    try {
      const response = await fetch('/api/rules', {
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
      setRules(data);
    } catch (error) {
      setRules({ error: (error as Error).message });
    }
  };

  const fetchDecoders = async (authToken: string, endpoint: string) => {
    try {
      const response = await fetch('/api/decoders', {
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
      setDecoders(data);
    } catch (error) {
      setDecoders({ error: (error as Error).message });
    }
  };

  const fetchManagerLogs = async (authToken: string, endpoint: string) => {
    try {
      const response = await fetch('/api/manager/logs', {
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
      setManagerLogs(data);
    } catch (error) {
      setManagerLogs({ error: (error as Error).message });
    }
  };

  const fetchManagerStats = async (authToken: string, endpoint: string) => {
    try {
      const response = await fetch('/api/manager/stats', {
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
      setManagerStats(data);
    } catch (error) {
      setManagerStats({ error: (error as Error).message });
    }
  };

  return (
    <main className="min-h-screen bg-white p-8">
      <div className="max-w-7xl mx-auto">
        <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
          <div className="md:col-span-1">
            <WazuhAuthForm onSubmit={handleSubmit} error={error} />
          </div>
          
          <div className="md:col-span-2 space-y-4">
            {token && (
              <CollapsibleSection title="JWT Token">
                <pre className="bg-gray-50 p-4 rounded-md overflow-auto text-xs text-gray-800">
                  {token}
                </pre>
              </CollapsibleSection>
            )}

            {agents && (
              <CollapsibleSection title="Agents">
                <pre className="bg-gray-50 p-4 rounded-md overflow-auto text-xs text-gray-800">
                  {JSON.stringify(agents, null, 2)}
                </pre>
              </CollapsibleSection>
            )}

            {clusterStatus && (
              <CollapsibleSection title="Cluster Status">
                <pre className="bg-gray-50 p-4 rounded-md overflow-auto text-xs text-gray-800">
                  {JSON.stringify(clusterStatus, null, 2)}
                </pre>
              </CollapsibleSection>
            )}

            {managerStatus && (
              <CollapsibleSection title="Manager Status">
                <pre className="bg-gray-50 p-4 rounded-md overflow-auto text-xs text-gray-800">
                  {JSON.stringify(managerStatus, null, 2)}
                </pre>
              </CollapsibleSection>
            )}

            {managerInfo && (
              <CollapsibleSection title="Manager Info">
                <pre className="bg-gray-50 p-4 rounded-md overflow-auto text-xs text-gray-800">
                  {JSON.stringify(managerInfo, null, 2)}
                </pre>
              </CollapsibleSection>
            )}

            {clusterLocalInfo && (
              <CollapsibleSection title="Cluster Local Info">
                <pre className="bg-gray-50 p-4 rounded-md overflow-auto text-xs text-gray-800">
                  {JSON.stringify(clusterLocalInfo, null, 2)}
                </pre>
              </CollapsibleSection>
            )}

            {rules && (
              <CollapsibleSection title="Rules">
                <pre className="bg-gray-50 p-4 rounded-md overflow-auto text-xs text-gray-800">
                  {JSON.stringify(rules, null, 2)}
                </pre>
              </CollapsibleSection>
            )}

            {decoders && (
              <CollapsibleSection title="Decoders">
                <pre className="bg-gray-50 p-4 rounded-md overflow-auto text-xs text-gray-800">
                  {JSON.stringify(decoders, null, 2)}
                </pre>
              </CollapsibleSection>
            )}

            {managerLogs && (
              <CollapsibleSection title="Manager Logs">
                <pre className="bg-gray-50 p-4 rounded-md overflow-auto text-xs text-gray-800">
                  {JSON.stringify(managerLogs, null, 2)}
                </pre>
              </CollapsibleSection>
            )}

            {managerStats && (
              <CollapsibleSection title="Manager Stats">
                <pre className="bg-gray-50 p-4 rounded-md overflow-auto text-xs text-gray-800">
                  {JSON.stringify(managerStats, null, 2)}
                </pre>
              </CollapsibleSection>
            )}
          </div>
        </div>
      </div>
    </main>
  );
}
