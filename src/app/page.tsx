'use client';
import { useState } from 'react';
import FloatingAuthHeader from './components/FloatingAuthHeader';
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
    <div className="relative min-h-screen bg-gray-50">
      <FloatingAuthHeader 
        onSubmit={handleSubmit} 
        error={error}
        isAuthenticated={!!agents}
      />
      
      <div className={`transition-all duration-300 ${agents ? 'pt-24' : 'pt-0'}`}>
        {agents ? (
          <div className="p-8">
            <div className="max-w-7xl mx-auto">
              <AgentsVisualization agents={agents} />
            </div>
          </div>
        ) : (
          <div className="flex items-center justify-center min-h-screen">
            <div className="text-center max-w-md mx-auto p-8">
              <h1 className="text-3xl font-bold text-gray-900 mb-4">
                Welcome to Wazuh Dashboard
              </h1>
              <p className="text-gray-600">
                Please authenticate using the form above to view your agent information.
              </p>
            </div>
          </div>
        )}
      </div>
    </div>
  );
}
