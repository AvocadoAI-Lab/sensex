'use client';
import { useState } from 'react';
import WazuhAuthForm from './components/WazuhAuthForm';
import CollapsibleSection from './components/CollapsibleSection';
import { AgentsResponse } from './types/agents';

export default function Home() {
  const [token, setToken] = useState<string | null>(null);
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
        setToken(data.token);
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
          </div>
        </div>
      </div>
    </main>
  );
}
