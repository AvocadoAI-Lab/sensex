import { describe, expect, test, beforeAll } from '@jest/globals';
import { makeAuthorizedRequest } from '../utils/auth-helper';
const fs = require('fs');
const path = require('path');

describe('Wazuh Agents API Through Rust Proxy', () => {
    let firstAgentId: string;
    let documentation = '# Wazuh Agents API Test Results\n\n';

    const appendToDoc = (section: string, content: any) => {
        documentation += `## ${section}\n\`\`\`json\n${JSON.stringify(content, null, 2)}\n\`\`\`\n\n`;
    };

    test('should proxy get all agents request', async () => {
        const response = await makeAuthorizedRequest('/agents');
        appendToDoc('All Agents Response', response);
        expect(response).toBeDefined();

        // Store first agent ID for subsequent tests
        if (response.data?.affected_items?.length > 0) {
            firstAgentId = response.data.affected_items[0].id;
            console.log('Found first agent ID:', firstAgentId);
        }
    });

    test('should proxy get specific agent details', async () => {
        // Skip if no agent ID available
        if (!firstAgentId) {
            console.log('No agents available to test');
            return;
        }

        console.log('Getting details for agent:', firstAgentId);
        const response = await makeAuthorizedRequest(`/agents?agents_list=${firstAgentId}`);
        appendToDoc(`Agent Details (ID: ${firstAgentId})`, response);
        expect(response).toBeDefined();
    });

    afterAll(() => {
        // Write documentation to file
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
