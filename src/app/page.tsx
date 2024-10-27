'use client';
import { useState } from 'react';
import WazuhAuthForm from './components/WazuhAuthForm';
import AgentsVisualization from './components/AgentsVisualization';
import { AgentsResponse } from './types/agents';

export default function HomePage() {
  const [agents, setAgents] = useState<AgentsResponse | null>(null);
  const [error, setError] = useState<string | null>(null);

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
        setError(null);
        fetchAgents(data.token, formData.endpoint);
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

  return (
    <main className="min-h-screen bg-gray-50 p-8">
      <div className="max-w-7xl mx-auto">
        <div className="grid grid-cols-1 md:grid-cols-4 gap-6">
          <div className="md:col-span-1">
            <WazuhAuthForm onSubmit={handleSubmit} error={error} />
          </div>
          
          <div className="md:col-span-3">
            {agents ? (
              <AgentsVisualization agents={agents} />
            ) : (
              <div className="bg-white p-8 rounded-lg shadow text-center">
                <h2 className="text-xl font-semibold text-gray-700">Welcome to Wazuh Dashboard</h2>
                <p className="mt-2 text-gray-600">Please authenticate to view agent information</p>
              </div>
            )}
          </div>
        </div>
      </div>
    </main>
  );
}
