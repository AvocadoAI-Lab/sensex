import { StatusType } from '../components/common/StatusIndicator';

export interface ConnectionStatus {
  status: StatusType;
  message: string;
}

export const getConnectionStatus = (isConnected: boolean, error?: string): ConnectionStatus => {
  if (error) {
    return {
      status: 'error',
      message: error
    };
  }
  
  return {
    status: isConnected ? 'success' : 'inactive',
    message: isConnected ? 'Connected' : 'Not connected'
  };
};
