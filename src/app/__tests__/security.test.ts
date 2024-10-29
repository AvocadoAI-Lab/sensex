import { describe, expect, test, beforeAll } from '@jest/globals';
import fs from "fs";

import path from "path";

const BASE_URL = "http://127.0.0.1:3001";
const WAZUH_URL = "https://wazuh.aixsoar.com:55000";

describe('Security Endpoints Through Rust Proxy', () => {
    let authToken: string;
    let documentation = '# Security Endpoints Test Results\n\n';

    const appendToDoc = (section: string, content: any) => {
        documentation += `## ${section}\n\`\`\`json\n${JSON.stringify(content, null, 2)}\n\`\`\`\n\n`;
    };

    beforeAll(async () => {
        // Get auth token first
        const authUrl = `${BASE_URL}/security/user/authenticate`;
        const response = await fetch(authUrl, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({
                endpoint: WAZUH_URL,
                token: "wazuh-wui:S.Ouv.51BHmQ*wqhq0O?eKSAyshu0Z.*"
            })
        });

        const data = await response.json();
        authToken = data.data?.token;
    });

    test('should proxy current user information request', async () => {
        const url = `${BASE_URL}/security/users/me`;
        const response = await fetch(url, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({
                endpoint: WAZUH_URL,
                token: authToken
            })
        });

        const data = await response.json();
        appendToDoc('Current User Information', data);
        expect(data).toBeDefined();
    });

    test('should proxy user policies request', async () => {
        const url = `${BASE_URL}/security/users/me/policies`;
        const response = await fetch(url, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({
                endpoint: WAZUH_URL,
                token: authToken
            })
        });

        const data = await response.json();
        appendToDoc('User Policies', data);
        expect(data).toBeDefined();
    });

    test('should proxy security roles request', async () => {
        const url = `${BASE_URL}/security/roles`;
        const response = await fetch(url, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({
                endpoint: WAZUH_URL,
                token: authToken
            })
        });

        const data = await response.json();
        appendToDoc('Security Roles', data);
        expect(data).toBeDefined();
    });

    test('should proxy security policies request', async () => {
        const url = `${BASE_URL}/security/policies`;
        const response = await fetch(url, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({
                endpoint: WAZUH_URL,
                token: authToken
            })
        });

        const data = await response.json();
        appendToDoc('Security Policies', data);
        expect(data).toBeDefined();
    });

    test('should proxy security rules request', async () => {
        const url = `${BASE_URL}/security/rules`;
        const response = await fetch(url, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({
                endpoint: WAZUH_URL,
                token: authToken
            })
        });

        const data = await response.json();
        appendToDoc('Security Rules', data);
        expect(data).toBeDefined();
    });

    afterAll(() => {
        // Write documentation to file
        const docsDir = path.join(__dirname, '..', 'docs');
        if (!fs.existsSync(docsDir)) {
            fs.mkdirSync(docsDir, { recursive: true });
        }
        fs.writeFileSync(
            path.join(docsDir, 'security-endpoints.md'),
            documentation
        );
    });
});
