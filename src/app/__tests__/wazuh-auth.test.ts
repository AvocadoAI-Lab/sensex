const nodeFetch = require('node-fetch');
const https = require('https');
const fs = require('fs');
const path = require('path');

import { BASE_URL, httpsAgent } from '../utils/auth-helper';

const USERNAME = "wazuh-wui";
const PASSWORD = "S.Ouv.51BHmQ*wqhq0O?eKSAyshu0Z.*";

interface WazuhAuthResponse {
    data?: {
        token: string;
    };
    token?: string;
}

describe('Wazuh Authentication Flow', () => {
    let authToken: string;

    // Create documentation
    let documentation = '# Wazuh Authentication API Test Results\n\n';
    const appendToDoc = (section: string, content: any) => {
        documentation += `## ${section}\n\`\`\`json\n${JSON.stringify(content, null, 2)}\n\`\`\`\n\n`;
    };

    const getAuthToken = async (): Promise<string> => {
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
            throw new Error(`Authentication failed: ${response.statusText}`);
        }

        const data = await response.json() as WazuhAuthResponse;
        appendToDoc('Auth Token Response', data);
        return data.token || (data.data && data.data.token) || '';
    };

    test('should authenticate successfully', async () => {
        const authUrl = `${BASE_URL}/security/user/authenticate`;
        const authString = Buffer.from(`${USERNAME}:${PASSWORD}`).toString('base64');
        
        const response = await nodeFetch(authUrl, {
            method: 'GET',
            headers: {
                'Authorization': `Basic ${authString}`
            },
            agent: httpsAgent
        });

        expect(response.status).toBe(200);
        const data = await response.json() as WazuhAuthResponse;
        appendToDoc('Successful Authentication Response', data);
        expect(data).toBeDefined();
    });

    test('should fail with incorrect credentials', async () => {
        const authUrl = `${BASE_URL}/security/user/authenticate`;
        const wrongAuthString = Buffer.from(`${USERNAME}:wrong_password`).toString('base64');
        
        const response = await nodeFetch(authUrl, {
            method: 'GET',
            headers: {
                'Authorization': `Basic ${wrongAuthString}`
            },
            agent: httpsAgent
        });

        expect(response.status).toBe(401);
        try {
            const errorData = await response.json();
            appendToDoc('Failed Authentication Response', errorData);
        } catch (e) {
            const errorText = await response.text();
            appendToDoc('Failed Authentication Response', { error: errorText });
        }
    });

    test('should obtain valid token', async () => {
        authToken = await getAuthToken();
        appendToDoc('Valid Token', { token: authToken });
        expect(authToken).toBeDefined();
        expect(typeof authToken).toBe('string');
        expect(authToken.length).toBeGreaterThan(0);
    });

    test('should access protected endpoint with token', async () => {
        if (!authToken) {
            authToken = await getAuthToken();
        }

        const testUrl = `${BASE_URL}/agents`;
        const response = await nodeFetch(testUrl, {
            method: 'GET',
            headers: {
                'Authorization': `Bearer ${authToken}`
            },
            agent: httpsAgent
        });

        expect(response.status).toBe(200);
        const data = await response.json();
        appendToDoc('Protected Endpoint Response', data);
        expect(data).toBeDefined();
    });

    afterAll(() => {
        // Write documentation to file
        const docsDir = path.join(__dirname, '..', 'docs');
        if (!fs.existsSync(docsDir)) {
            fs.mkdirSync(docsDir, { recursive: true });
        }
        fs.writeFileSync(
            path.join(docsDir, 'wazuh-auth-responses.md'),
            documentation
        );
    });
});
