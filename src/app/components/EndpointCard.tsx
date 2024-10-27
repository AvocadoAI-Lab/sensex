import { useState, useEffect, useRef } from 'react';
import { WazuhResponse } from '../types/responses';

interface EndpointCardProps {
  title: string;
  response: WazuhResponse | null;
}

export default function EndpointCard({ title, response }: EndpointCardProps) {
  const [showModal, setShowModal] = useState(false);
  const modalRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    function handleClickOutside(event: MouseEvent) {
      if (modalRef.current && !modalRef.current.contains(event.target as Node)) {
        setShowModal(false);
      }
    }

    if (showModal) {
      document.addEventListener('mousedown', handleClickOutside);
    }

    return () => {
      document.removeEventListener('mousedown', handleClickOutside);
    };
  }, [showModal]);

  const getStatusColor = () => {
    if (!response) return 'bg-gray-400'; // Not fetched
    if (response.error) return 'bg-red-500'; // Error
    return 'bg-green-500'; // Success
  };

  return (
    <>
      <div 
        className="bg-white rounded-lg shadow-md p-4 cursor-pointer hover:shadow-lg transition-shadow"
        onClick={() => setShowModal(true)}
      >
        <div className="flex items-center justify-between">
          <h3 className="text-sm font-medium text-gray-900">{title}</h3>
          <div className={`w-3 h-3 rounded-full ${getStatusColor()}`} />
        </div>
      </div>

      {showModal && response && (
        <>
          {/* Backdrop */}
          <div className="fixed inset-0 bg-black bg-opacity-50 z-40" />
          
          {/* Modal */}
          <div className="fixed inset-0 z-50 flex items-center justify-center p-4">
            <div 
              ref={modalRef}
              className="bg-white rounded-lg shadow-xl max-w-2xl w-full max-h-[90vh] flex flex-col"
            >
              <div className="flex justify-between items-center p-4 border-b">
                <h4 className="text-lg font-medium text-gray-900">{title} Details</h4>
                <button 
                  onClick={() => setShowModal(false)}
                  className="text-gray-400 hover:text-gray-500 focus:outline-none"
                >
                  <svg className="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" />
                  </svg>
                </button>
              </div>
              
              <div className="flex-1 overflow-auto p-4">
                <pre className="text-sm bg-gray-50 p-4 rounded overflow-auto">
                  {JSON.stringify(response, null, 2)}
                </pre>
              </div>
            </div>
          </div>
        </>
      )}
    </>
  );
}
