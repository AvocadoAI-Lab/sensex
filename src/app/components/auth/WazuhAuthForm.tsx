import { useState } from 'react';
import StatusIndicator from '../common/StatusIndicator';
import { getConnectionStatus } from '../../utils/status';

interface FormData {
  endpoint: string;
  username: string;
  password: string;
}

interface WazuhAuthFormProps {
  onSubmit: (formData: FormData) => Promise<void>;
  error: string | null;
  isConnected: boolean;
}

export default function WazuhAuthForm({ onSubmit, error, isConnected }: WazuhAuthFormProps) {
  const [formData, setFormData] = useState({
    endpoint: '',
    username: '',
    password: '',
  });

  const connectionStatus = getConnectionStatus(isConnected, error || undefined);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    await onSubmit(formData);
  };

  const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setFormData({
      ...formData,
      [e.target.name]: e.target.value,
    });
  };

  return (
    <div className="bg-white rounded-xl shadow-md overflow-hidden p-6 h-fit border border-gray-100">
      <div className="flex justify-between items-center mb-6">
        <h1 className="text-2xl font-bold text-gray-800">Wazuh Authentication</h1>
        <StatusIndicator 
          status={connectionStatus.status} 
          text={connectionStatus.message} 
        />
      </div>
      
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
            className="mt-1 block w-full rounded-md border-gray-200 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm p-2 border bg-white text-gray-800"
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
            className="mt-1 block w-full rounded-md border-gray-200 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm p-2 border bg-white text-gray-800"
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
            className="mt-1 block w-full rounded-md border-gray-200 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm p-2 border bg-white text-gray-800"
            required
          />
        </div>

        <button
          type="submit"
          className="w-full flex justify-center py-2 px-4 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-blue-500 hover:bg-blue-600 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500"
        >
          Connect
        </button>
      </form>

      {error && (
        <div className="mt-4 p-4 bg-red-50 text-red-700 rounded-md border border-red-100">
          <p className="text-sm">{error}</p>
        </div>
      )}
    </div>
  );
}
