import { useState } from 'react';
import { WazuhResponse } from '../types/responses';

interface EndpointCardProps {
  title: string;
  response: WazuhResponse | null;
}

export default function EndpointCard({ title, response }: EndpointCardProps) {
  const [showPopover, setShowPopover] = useState(false);

  const getStatusColor = () => {
    if (!response) return 'bg-gray-400'; // Not fetched
    if (response.error) return 'bg-red-500'; // Error
    return 'bg-green-500'; // Success
  };

  return (
    <div className="relative">
      <div 
        className="bg-white rounded-lg shadow-md p-4 cursor-pointer hover:shadow-lg transition-shadow"
        onClick={() => setShowPopover(!showPopover)}
      >
        <div className="flex items-center justify-between">
          <h3 className="text-sm font-medium text-gray-900">{title}</h3>
          <div className={`w-3 h-3 rounded-full ${getStatusColor()}`} />
        </div>
      </div>

      {showPopover && response && (
        <div className="absolute z-10 w-96 mt-2 bg-white rounded-lg shadow-xl p-4 right-0">
          <div className="flex justify-between items-center mb-2">
            <h4 className="text-sm font-medium text-gray-900">{title} Details</h4>
            <button 
              onClick={(e) => {
                e.stopPropagation();
                setShowPopover(false);
              }}
              className="text-gray-400 hover:text-gray-500"
            >
              ✕
            </button>
          </div>
          <pre className="text-xs bg-gray-50 p-2 rounded overflow-auto max-h-96">
            {JSON.stringify(response, null, 2)}
          </pre>
        </div>
      )}
    </div>
  );
}
