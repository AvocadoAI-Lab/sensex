'use client';
import { useState } from 'react';
import WazuhAuthForm from '../components/WazuhAuthForm';
import EndpointGroup from '../components/EndpointGroup';
import { useWazuhEndpoints } from '../hooks/useWazuhEndpoints';
import { endpointGroups } from '../config/endpoints';

export default function DevPage() {
  const [token, setToken] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);
  const { responses, fetchAllEndpoints } = useWazuhEndpoints();

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
        // Fetch all endpoints after authentication
        await fetchAllEndpoints(data.token, formData.endpoint);
      } else {
        setError(data.error || 'Authentication failed');
      }
    } catch (error) {
      setError((error as Error).message);
    }
  };

  return (
    <main className="min-h-screen bg-gray-100 p-8">
      <div className="max-w-7xl mx-auto">
        <div className="grid grid-cols-1 md:grid-cols-4 gap-6">
          <div className="md:col-span-1">
            <WazuhAuthForm onSubmit={handleSubmit} error={error} />
            
            {token && (
              <div className="mt-4 bg-white p-4 rounded-lg shadow">
                <h3 className="text-sm font-medium text-gray-900 mb-2">JWT Token</h3>
                <pre className="text-xs bg-gray-50 p-2 rounded overflow-auto max-h-32">
                  {token}
                </pre>
              </div>
            )}
          </div>
          
          <div className="md:col-span-3 space-y-6">
            {token && endpointGroups.map((group, index) => (
              <EndpointGroup
                key={index}
                title={group.title}
                endpoints={group.endpoints}
                responses={responses}
              />
            ))}
          </div>
        </div>
      </div>
    </main>
  );
}
