export interface Agent {
  id: string;
  name: string;
  ip: string;
  status: string;
  dateAdd: string;
  version: string;
  manager: string;
  os: {
    arch: string;
    platform: string;
    version: string;
  };
}

export interface AgentsResponse {
  data: {
    affected_items: Agent[];
    total_affected_items: number;
  };
}
