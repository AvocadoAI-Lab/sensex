'use client';
import { useState } from 'react';

export default function Home() {
  const [formData, setFormData] = useState({
    endpoint: '',
    username: '',
    password: '',
  });
  const [token, setToken] = useState<string | null>(null);
  const [agents, setAgents] = useState<any>(null);
  const [error, setError] = useState<string | null>(null);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
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
        // 自動獲取 agents
        fetchAgents(data.token);
      } else {
        setError(data.error || 'Authentication failed');
      }
    } catch (error) {
      setError((error as Error).message);
    }
  };

  const fetchAgents = async (authToken: string) => {
    try {
      const response = await fetch('/api/agents', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          endpoint: formData.endpoint,
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

  const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setFormData({
      ...formData,
      [e.target.name]: e.target.value,
    });
  };

  return (
    <main className="min-h-screen p-8">
      <div className="max-w-md mx-auto bg-white rounded-xl shadow-md overflow-hidden md:max-w-2xl p-6">
        <h1 className="text-2xl font-bold mb-6">Wazuh Authentication</h1>
        
        <form onSubmit={handleSubmit} className="space-y-4">
          <div>
            <label htmlFor="endpoint" className="block text-sm font-medium text-gray-700">
              Wazuh Endpoint
            </label>
            <input
              type="text"
              id="endpoint"
              name="endpoint"
              value={formData.endpoint}
              onChange={handleChange}
              className="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:text-sm p-2 border"
              placeholder="https://your-wazuh-endpoint:55000"
              required
            />
          </div>

          <div>
            <label htmlFor="username" className="block text-sm font-medium text-gray-700">
              Username
            </label>
            <input
              type="text"
              id="username"
              name="username"
              value={formData.username}
              onChange={handleChange}
              className="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:text-sm p-2 border"
              required
            />
          </div>

          <div>
            <label htmlFor="password" className="block text-sm font-medium text-gray-700">
              Password
            </label>
            <input
              type="password"
              id="password"
              name="password"
              value={formData.password}
              onChange={handleChange}
              className="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:text-sm p-2 border"
              required
            />
          </div>

          <button
            type="submit"
            className="w-full flex justify-center py-2 px-4 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500"
          >
            Connect
          </button>
        </form>

        {error && (
          <div className="mt-4 p-4 bg-red-50 text-red-700 rounded-md">
            <p className="text-sm">{error}</p>
          </div>
        )}

        {token && (
          <div className="mt-4">
            <h2 className="text-lg font-semibold mb-2">JWT Token:</h2>
            <pre className="bg-gray-100 p-4 rounded-md overflow-auto text-xs">
              {token}
            </pre>
          </div>
        )}

        {agents && (
          <div className="mt-4">
            <h2 className="text-lg font-semibold mb-2">Agents:</h2>
            <pre className="bg-gray-100 p-4 rounded-md overflow-auto text-xs">
              {JSON.stringify(agents, null, 2)}
            </pre>
          </div>
        )}
      </div>
    </main>
  );
}
