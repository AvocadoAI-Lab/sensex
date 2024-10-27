import { ReactNode } from 'react';

export type StatusType = 'success' | 'error' | 'warning' | 'inactive';

interface StatusIndicatorProps {
  status: StatusType;
  text?: string;
  children?: ReactNode;
}

const statusColors = {
  success: 'bg-green-500',
  error: 'bg-red-500',
  warning: 'bg-yellow-500',
  inactive: 'bg-gray-300'
};

export default function StatusIndicator({ status, text, children }: StatusIndicatorProps) {
  return (
    <div className="flex items-center space-x-2">
      <div className={`w-3 h-3 rounded-full ${statusColors[status]} animate-pulse`} />
      {text && <span className="text-sm text-gray-600">{text}</span>}
      {children}
    </div>
  );
}
