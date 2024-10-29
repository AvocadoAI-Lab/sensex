import { describe, expect, test } from '@jest/globals';
import fs from "fs";

import path from "path";
// Point to our Rust proxy
const BASE_URL = "http://127.0.0.1:3001";
const WAZUH_URL = "https://wazuh.aixsoar.com:55000";

interface WazuhAuthResponse {
    data?: {
        token: string;
    };
    error?: string;
}

describe('Wazuh Authentication Flow Through Rust Proxy', () => {
    let authToken: string;

    // Create documentation
    let documentation = '# Wazuh Authentication API Test Results (Through Rust Proxy)\n\n';
    const appendToDoc = (section: string, content: any) => {
        documentation += `## ${section}\n\`\`\`json\n${JSON.stringify(content, null, 2)}\n\`\`\`\n\n`;
    };

    const getAuthToken = async (): Promise<string> => {
        const authUrl = `${BASE_URL}/security/user/authenticate`;
        
        // Send username:password in token field
        const payload = {
            endpoint: WAZUH_URL,
            token: "wazuh-wui:S.Ouv.51BHmQ*wqhq0O?eKSAyshu0Z.*"
        };

        const response = await fetch(authUrl, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify(payload)
        });

        if (!response.ok) {
            const errorText = await response.text();
            throw new Error(`Authentication failed: ${response.statusText}. ${errorText}`);
        }

        const data = await response.json() as WazuhAuthResponse;
        appendToDoc('Auth Token Response Through Proxy', data);
        
        if (!data.data?.token) {
            throw new Error(data.error || 'No token received in response');
        }
        
        return data.data.token;
    };

    test('should authenticate successfully through proxy', async () => {
        const authUrl = `${BASE_URL}/security/user/authenticate`;
        
        const payload = {
            endpoint: WAZUH_URL,
            token: "wazuh-wui:S.Ouv.51BHmQ*wqhq0O?eKSAyshu0Z.*"
        };

        const response = await fetch(authUrl, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify(payload)
        });

        expect(response.status).toBe(200);
        const data = await response.json() as WazuhAuthResponse;
        appendToDoc('Successful Authentication Response Through Proxy', data);
        expect(data.data?.token).toBeDefined();
        expect(typeof data.data?.token).toBe('string');
        expect(data.data?.token.length).toBeGreaterThan(0);
    });

    test('should fail with incorrect credentials through proxy', async () => {
        const authUrl = `${BASE_URL}/security/user/authenticate`;
        
        const payload = {
            endpoint: WAZUH_URL,
            token: "wazuh-wui:wrong_password"
        };

        const response = await fetch(authUrl, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify(payload)
        });

        expect(response.status).toBe(401);
        const data = await response.json() as WazuhAuthResponse;
        appendToDoc('Failed Authentication Response Through Proxy', data);
        expect(data.error).toBeDefined();
    });

    test('should obtain valid token through proxy', async () => {
        authToken = await getAuthToken();
        appendToDoc('Valid Token Through Proxy', { token: authToken });
        expect(authToken).toBeDefined();
        expect(typeof authToken).toBe('string');
        expect(authToken.length).toBeGreaterThan(0);
    });

    test('should access protected endpoint through proxy with token', async () => {
        if (!authToken) {
            authToken = await getAuthToken();
        }

        const testUrl = `${BASE_URL}/security/users/me`;
        const response = await fetch(testUrl, {
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
        appendToDoc('Protected Endpoint Response Through Proxy', data);
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
