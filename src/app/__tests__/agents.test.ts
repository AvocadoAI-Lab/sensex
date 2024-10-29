import { makeAuthorizedRequest } from '../utils/auth-helper';
import fs from 'fs';
import path from 'path';
import type { WazuhResponse } from '../types/responses';

describe('Wazuh Agents API Through Rust Proxy', () => {
    let firstAgentId: string;
    let documentation = '# Wazuh Agents API Test Results\n\n';

    const appendToDoc = (section: string, content: WazuhResponse): void => {
        documentation += `## ${section}\n\`\`\`json\n${JSON.stringify(content, null, 2)}\n\`\`\`\n\n`;
    };

    test('should proxy get all agents request', async () => {
        const response = await makeAuthorizedRequest('/agents');
        
        expect(response).toBeDefined();
        if (!response) {
            throw new Error('Response is null');
        }

        const typedResponse = response as WazuhResponse;
        appendToDoc('All Agents Response', typedResponse);

        // Store first agent ID for subsequent tests
        if (typedResponse.data?.affected_items?.length > 0) {
            const firstAgent = typedResponse.data.affected_items[0];
            if (!firstAgent.id) {
                throw new Error('Agent ID is undefined');
            }
            firstAgentId = firstAgent.id;
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
        
        expect(response).toBeDefined();
        if (!response) {
            throw new Error('Response is null');
        }

        const typedResponse = response as WazuhResponse;
        appendToDoc(`Agent Details (ID: ${firstAgentId})`, typedResponse);
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
