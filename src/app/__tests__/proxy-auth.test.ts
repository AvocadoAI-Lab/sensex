const nodeFetch = require('node-fetch');

const PROXY_URL = "http://127.0.0.1:3001";
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
        const authString = Buffer.from(`${USERNAME}:${PASSWORD}`).toString('base64');
        
        const response = await nodeFetch(authUrl, {
            method: 'GET',
            headers: {
                'Authorization': `Basic ${authString}`
            }
        });

        if (!response.ok) {
            throw new Error(`Authentication failed: ${response.statusText}`);
        }

        const data = await response.json() as WazuhAuthResponse;
        return data.token || (data.data && data.data.token) || '';
    };

    test('should authenticate through Rust proxy', async () => {
        const authUrl = `${PROXY_URL}/security/user/authenticate`;
        const authString = Buffer.from(`${USERNAME}:${PASSWORD}`).toString('base64');
        
        const response = await nodeFetch(authUrl, {
            method: 'GET',
            headers: {
                'Authorization': `Basic ${authString}`
            }
        });

        expect(response.status).toBe(200);
        const data = await response.json() as WazuhAuthResponse;
        expect(data).toBeDefined();
        console.log('Proxy Auth Response:', data);
    });

    test('should fail with incorrect credentials through proxy', async () => {
        const authUrl = `${PROXY_URL}/security/user/authenticate`;
        const wrongAuthString = Buffer.from(`${USERNAME}:wrong_password`).toString('base64');
        
        const response = await nodeFetch(authUrl, {
            method: 'GET',
            headers: {
                'Authorization': `Basic ${wrongAuthString}`
            }
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
            method: 'GET',
            headers: {
                'Authorization': `Bearer ${authToken}`
            }
        });

        expect(response.status).toBe(200);
        const data = await response.json();
        expect(data).toBeDefined();
        console.log('Protected Endpoint Response through proxy:', data);
    });
});
