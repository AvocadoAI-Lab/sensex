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
    <div className={`absolute top-0 right-0 transition-all duration-300 ease-in-out ${isAuthenticated ? 'w-80' : 'w-full'}`}>
      <div className="bg-white shadow-lg m-4 rounded-xl overflow-hidden">
        <div className="p-4">
          {isAuthenticated ? (
            <div className="flex items-center justify-between">
              <div className="flex items-center space-x-2">
                <div className="w-2 h-2 bg-green-500 rounded-full"></div>
                <span className="text-sm font-medium text-gray-700">Connected to Wazuh</span>
              </div>
              <button
                onClick={() => setIsExpanded(!isExpanded)}
                className="p-1 rounded-lg hover:bg-gray-100 transition-colors"
              >
                <svg
                  className={`w-5 h-5 text-gray-500 transform transition-transform ${isExpanded ? 'rotate-180' : ''}`}
                  fill="none"
                  stroke="currentColor"
                  viewBox="0 0 24 24"
                >
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    strokeWidth={2}
                    d={isExpanded ? "M5 15l7-7 7 7" : "M19 9l-7 7-7-7"}
                  />
                </svg>
              </button>
            </div>
          ) : null}
          
          <div className={`overflow-hidden transition-all duration-300 ${
            isAuthenticated && !isExpanded ? 'max-h-0' : 'max-h-[600px]'
          }`}>
            {isAuthenticated && <div className="h-4"></div>}
            <WazuhAuthForm onSubmit={onSubmit} error={error} />
          </div>
        </div>
      </div>
    </div>
  );
}
