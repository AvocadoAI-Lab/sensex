const nodeFetch = require('node-fetch');
const https = require('https');

const BASE_URL = "https://wazuh.aixsoar.com:55000";
const USERNAME = "wazuh-wui";
const PASSWORD = "S.Ouv.51BHmQ*wqhq0O?eKSAyshu0Z.*";

// Create an HTTPS agent that ignores SSL certificate validation
const httpsAgent = new https.Agent({
    rejectUnauthorized: false
});

interface WazuhAuthResponse {
    data?: {
        token: string;
    };
    token?: string;
}

export async function getAuthToken(): Promise<string> {
    const authUrl = `${BASE_URL}/security/user/authenticate`;
    const authString = Buffer.from(`${USERNAME}:${PASSWORD}`).toString('base64');
    
    const response = await nodeFetch(authUrl, {
        method: 'GET',
        headers: {
            'Authorization': `Basic ${authString}`
        },
        agent: httpsAgent
    });

    if (!response.ok) {
        const errorText = await response.text();
        throw new Error(`Authentication failed: ${response.statusText}. ${errorText}`);
    }

    const data = await response.json() as WazuhAuthResponse;
    return data.token || (data.data && data.data.token) || '';
}

export async function makeAuthorizedRequest(
    endpoint: string, 
    method: string = 'GET', 
    body?: any
): Promise<any> {
    const token = await getAuthToken();
    const url = `${BASE_URL}${endpoint}`;
    
    console.log(`Making ${method} request to: ${url}`);
    
    const response = await nodeFetch(url, {
        method,
        headers: {
            'Authorization': `Bearer ${token}`,
            'Content-Type': 'application/json'
        },
        body: body ? JSON.stringify(body) : undefined,
        agent: httpsAgent
    });

    const responseText = await response.text();
    
    if (!response.ok) {
        console.error(`Request failed: ${response.status} ${response.statusText}`);
        console.error('Response:', responseText);
        throw new Error(`Request failed: ${response.status} ${response.statusText}\nResponse: ${responseText}`);
    }

    try {
        return JSON.parse(responseText);
    } catch (e) {
        console.error('Failed to parse response as JSON:', responseText);
        throw e;
    }
}

export { BASE_URL, httpsAgent };
