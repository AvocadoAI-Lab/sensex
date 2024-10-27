'use client';
import { useState } from 'react';
import WazuhAuthForm from './WazuhAuthForm';

interface FloatingAuthHeaderProps {
  onSubmit: (formData: { endpoint: string; username: string; password: string; }) => Promise<void>;
  error: string | null;
  isAuthenticated: boolean;
}

export default function FloatingAuthHeader({ onSubmit, error, isAuthenticated }: FloatingAuthHeaderProps) {
  const [isExpanded, setIsExpanded] = useState(!isAuthenticated);

  return (
    <div className={`absolute top-0 right-0 transition-all duration-300 ease-in-out ${isAuthenticated ? 'w-64' : 'w-full'}`}>
      <div className="bg-white shadow-lg m-4 rounded-lg">
        <div className="p-4">
          {isAuthenticated ? (
            <div className="flex items-center justify-between">
              <span className="text-sm text-green-600 font-medium">Connected to Wazuh</span>
              <button
                onClick={() => setIsExpanded(!isExpanded)}
                className="text-gray-500 hover:text-gray-700"
              >
                {isExpanded ? '−' : '+'}
              </button>
            </div>
          ) : null}
          
          <div className={`overflow-hidden transition-all duration-300 ${isAuthenticated && !isExpanded ? 'max-h-0' : 'max-h-96'}`}>
            <WazuhAuthForm onSubmit={onSubmit} error={error} />
          </div>
        </div>
      </div>
    </div>
  );
}
