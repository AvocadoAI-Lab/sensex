export interface AgentOS {
    arch?: string;
    codename?: string;
    major?: string;
    minor?: string;
    name?: string;
    platform?: string;
    uname?: string;
    version?: string;
    [key: string]: unknown;  // 允許其他未定義的欄位
}

export interface Agent {
    dateAdd?: string;
    group_config_status?: string;
    id?: string;
    ip?: string;
    lastKeepAlive?: string;
    manager?: string;
    name?: string;
    node_name?: string;
    os?: AgentOS;
    registerIP?: string;
    status?: string;
    status_code?: number;
    version?: string;
    [key: string]: unknown;  // 允許其他未定義的欄位
}

export interface AgentsResponseData {
    affected_items: Agent[];
    failed_items?: unknown[];
    total_affected_items?: number;
    total_failed_items?: number;
    [key: string]: unknown;  // 允許其他未定義的欄位
}

export interface AgentsResponse {
    data: AgentsResponseData;
    error?: number;
    message?: string;
    [key: string]: unknown;  // 允許其他未定義的欄位
}

// Type guards for runtime type checking
export function isAgentOS(obj: unknown): obj is AgentOS {
    return typeof obj === 'object' && obj !== null;
}

export function isAgent(obj: unknown): obj is Agent {
    return typeof obj === 'object' && obj !== null;
}

export function isAgentsResponse(obj: unknown): obj is AgentsResponse {
    const response = obj as AgentsResponse;
    return (
        typeof response === 'object' &&
        response !== null &&
        typeof response.data === 'object' &&
        response.data !== null &&
        Array.isArray(response.data.affected_items)
    );
}

// Example usage:
/*
const response = await makeAuthorizedRequest('/agents');
if (isAgentsResponse(response)) {
    // TypeScript now knows this is an AgentsResponse
    const agents = response.data.affected_items;
    agents.forEach(agent => {
        if (agent.name && agent.os?.platform) {
            console.log(agent.name, agent.os.platform);
        }
    });
}
*/
