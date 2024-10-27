import { makeAuthorizedRequest } from '../utils/auth-helper';

interface Agent {
    id: string;
    status: string;
    name: string;
    ip: string;
    dateAdd: string;
    version: string;
    manager: string;
    os?: {
        arch?: string;
        platform?: string;
        version?: string;
    };
}

interface WazuhResponse<T> {
    data: {
        affected_items: T[];
        total_affected_items: number;
        total_failed_items: number;
        failed_items: any[];
    };
    error: number;
}

describe('Wazuh Agents API Flow', () => {
    let firstAgentId: string;

    // Create documentation
    let documentation = '# Wazuh Agents API Test Results\n\n';
    const appendToDoc = (section: string, content: any) => {
        documentation += `## ${section}\n\`\`\`json\n${JSON.stringify(content, null, 2)}\n\`\`\`\n\n`;
    };

    test('should get all agents', async () => {
        const response = await makeAuthorizedRequest('/agents') as WazuhResponse<Agent>;
        
        expect(response.data).toBeDefined();
        expect(Array.isArray(response.data.affected_items)).toBe(true);
        
        appendToDoc('All Agents Response', response);

        // Store first agent ID for subsequent tests
        if (response.data.affected_items.length > 0) {
            firstAgentId = response.data.affected_items[0].id;
            console.log('Found first agent ID:', firstAgentId);
        } else {
            console.log('No agents found in response');
        }
    }, 30000); // Increase timeout for this test

    test('should get specific agent details', async () => {
        // Skip if no agent ID available
        if (!firstAgentId) {
            console.log('No agents available to test');
            return;
        }

        console.log('Getting details for agent:', firstAgentId);
        const response = await makeAuthorizedRequest(`/agents?agents_list=${firstAgentId}`);
        
        expect(response.data).toBeDefined();
        expect(response.data.affected_items).toBeDefined();
        expect(response.data.affected_items.length).toBeGreaterThan(0);
        
        appendToDoc(`Agent Details (ID: ${firstAgentId})`, response);
    }, 30000);

    test('should get agent configuration', async () => {
        // Skip if no agent ID available
        if (!firstAgentId) {
            console.log('No agents available to test');
            return;
        }

        console.log('Getting configuration for agent:', firstAgentId);
        const response = await makeAuthorizedRequest(`/syscollector/${firstAgentId}/hardware`);
        
        expect(response).toBeDefined();
        appendToDoc(`Agent Hardware Info (ID: ${firstAgentId})`, response);
    }, 30000);

    test('should get agents summary', async () => {
        const response = await makeAuthorizedRequest('/agents/summary/status');
        
        expect(response.data).toBeDefined();
        appendToDoc('Agents Summary', response);
    }, 30000);

    afterAll(() => {
        // Write documentation to file
        const fs = require('fs');
        const path = require('path');
        const docsDir = path.join(__dirname, '..', 'docs');
        
        if (!fs.existsSync(docsDir)) {
            fs.mkdirSync(docsDir, { recursive: true });
        }
        
        fs.writeFileSync(
            path.join(docsDir, 'wazuh-agents-responses.md'),
            documentation
        );
    });
});
