export interface Agent {
  id: string;
  name: string;
  ip: string;
  status: string;
  status_code: number;
  dateAdd: string;
  version: string;
  manager: string;
  node_name: string;
  group?: string[];
  group_config_status: string;
  lastKeepAlive: string;
  disconnection_time?: string;
  registerIP: string;
  configSum?: string;
  mergedSum?: string;
  os: {
    arch?: string;
    codename?: string;
    major: string;
    minor: string;
    name: string;
    platform: string;
    uname: string;
    version: string;
    build?: string;
  };
}

export interface AgentsResponse {
  data: {
    affected_items: Agent[];
    total_affected_items: number;
    failed_items: any[];
    total_failed_items: number;
  };
  error: number;
  message: string;
}
