import nodeFetch from "node-fetch";

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

describe('Rust Proxy Authentication Flow', () => {
    let authToken: string;

    const getAuthToken = async (): Promise<string> => {
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
            throw new Error(`Authentication failed: ${response.statusText}. ${await response.text()}`);
        }

        const data = await response.json() as WazuhAuthResponse;
        return data.data?.token || '';
    };

    test('should authenticate through Rust proxy', async () => {
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

        expect(response.status).toBe(200);
        const data = await response.json() as WazuhAuthResponse;
        expect(data).toBeDefined();
        expect(data.data?.token).toBeDefined();
        console.log('Proxy Auth Response:', data);
    });

    test('should fail with incorrect credentials through proxy', async () => {
        const authUrl = `${PROXY_URL}/security/user/authenticate`;
        
        const response = await nodeFetch(authUrl, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({
                endpoint: WAZUH_URL,
                token: `${USERNAME}:wrong_password`
            })
        });

        expect(response.status).toBe(401);
        const errorData = await response.text();
        console.log('Failed Proxy Auth Response:', errorData);
    });

    test('should obtain valid token through proxy', async () => {
        authToken = await getAuthToken();
        expect(authToken).toBeDefined();
        expect(typeof authToken).toBe('string');
        expect(authToken.length).toBeGreaterThan(0);
        console.log('Proxy Auth Token:', authToken);
    });

    test('should access protected endpoint through proxy with token', async () => {
        if (!authToken) {
            authToken = await getAuthToken();
        }

        const testUrl = `${PROXY_URL}/agents`;
        const response = await nodeFetch(testUrl, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({
                endpoint: WAZUH_URL,
                token: authToken
            })
        });

        expect(response.status).toBe(200);
        const data = await response.json();
        expect(data).toBeDefined();
        console.log('Protected Endpoint Response through proxy:', data);
    });
});
