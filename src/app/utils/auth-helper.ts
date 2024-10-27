const nodeFetch = require('node-fetch');

const PROXY_URL = "http://127.0.0.1:3001";
const WAZUH_URL = "https://wazuh.aixsoar.com:55000";
const USERNAME = "wazuh-wui";
const PASSWORD = "S.Ouv.51BHmQ*wqhq0O?eKSAyshu0Z.*";

interface WazuhAuthResponse {
    data?: {
        token: string;
    };
    token?: string;
}

export async function getAuthToken(): Promise<string> {
    const authUrl = `${PROXY_URL}/security/user/authenticate`;
    
    const response = await nodeFetch(authUrl, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify({
            endpoint: WAZUH_URL,
            token: `${USERNAME}:${PASSWORD}`
        })
    });

    if (!response.ok) {
        const errorText = await response.text();
        throw new Error(`Authentication failed: ${response.statusText}. ${errorText}`);
    }

    const data = await response.json() as WazuhAuthResponse;
    return data.data?.token || '';
}

export async function makeAuthorizedRequest(
    endpoint: string, 
    method: string = 'GET',  // Keep for backward compatibility
    body?: any  // Keep for backward compatibility
): Promise<any> {
    const token = await getAuthToken();
    const url = `${PROXY_URL}${endpoint}`;
    
    console.log(`Making request to: ${url}`);

    try {
        const response = await nodeFetch(url, {
            method: 'POST',  // Always use POST for Rust backend
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({
                endpoint: WAZUH_URL,
                token: token
            })
        });

        if (!response.ok) {
            const errorText = await response.text();
            throw new Error(`Request failed: ${response.status} ${response.statusText}. ${errorText}`);
        }

        const text = await response.text();
        if (!text) {
            return null;  // Handle empty responses
        }

        const data = JSON.parse(text);
        console.log(`Response from ${url}:`, data);
        return data;
    } catch (error) {
        console.error(`Error making request to ${url}:`, error);
        throw error;
    }
}

export { PROXY_URL, WAZUH_URL };
