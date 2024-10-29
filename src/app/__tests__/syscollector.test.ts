import { makeAuthorizedRequest } from '../utils/auth-helper';
import fs from 'fs';
import path from 'path';
import type { WazuhResponse } from '../types/responses';

type DocContent = WazuhResponse | { message: string; id?: string } | { message?: string; id: string };

describe('Wazuh Syscollector API Flow', () => {
    // Create documentation
    let documentation = '# Wazuh Syscollector API Test Results\n\n';
    const appendToDoc = (section: string, content: DocContent): void => {
        documentation += `## ${section}\n\`\`\`json\n${JSON.stringify(content, null, 2)}\n\`\`\`\n\n`;
    };

    let agentId: string;

    beforeAll(async () => {
        // Get first available agent ID
        const response = await makeAuthorizedRequest('/agents');
        
        expect(response).toBeDefined();
        if (!response) {
            throw new Error('Response is null');
        }

        const typedResponse = response as WazuhResponse;
        if (typedResponse.data.affected_items.length > 0) {
            const firstAgent = typedResponse.data.affected_items[0];
            if (!firstAgent.id) {
                throw new Error('Agent ID is undefined');
            }
            // Get the first agent ID and ensure it's at least 3 digits
            agentId = firstAgent.id.padStart(3, '0');
            console.log('Using agent ID:', agentId);
            appendToDoc('Selected Agent', { id: agentId, message: 'Agent selected for testing' });
        } else {
            console.log('No agents available for testing');
        }
    });

    test('should get hardware info for agent', async () => {
        if (!agentId) {
            console.log('No agent available, skipping hardware test');
            appendToDoc('Hardware Info', { message: 'Test skipped - No agent available' });
            return;
        }

        const response = await makeAuthorizedRequest(`/syscollector/${agentId}/hardware`, 'POST', {
            params: {
                agent_id: agentId
            }
        });
        
        expect(response).toBeDefined();
        if (!response) {
            throw new Error('Response is null');
        }

        const typedResponse = response as WazuhResponse;
        expect(typedResponse.data).toBeDefined();
        
        appendToDoc('Hardware Info', typedResponse);
    }, 30000);

    test('should get OS info for agent', async () => {
        if (!agentId) {
            console.log('No agent available, skipping OS test');
            appendToDoc('OS Info', { message: 'Test skipped - No agent available' });
            return;
        }

        const response = await makeAuthorizedRequest(`/syscollector/${agentId}/os`, 'POST', {
            params: {
                agent_id: agentId
            }
        });
        
        expect(response).toBeDefined();
        if (!response) {
            throw new Error('Response is null');
        }

        const typedResponse = response as WazuhResponse;
        expect(typedResponse.data).toBeDefined();
        
        appendToDoc('OS Info', typedResponse);
    }, 30000);

    test('should get packages info for agent', async () => {
        if (!agentId) {
            console.log('No agent available, skipping packages test');
            appendToDoc('Packages Info', { message: 'Test skipped - No agent available' });
            return;
        }

        const response = await makeAuthorizedRequest(`/syscollector/${agentId}/packages`, 'POST', {
            params: {
                agent_id: agentId
            }
        });
        
        expect(response).toBeDefined();
        if (!response) {
            throw new Error('Response is null');
        }

        const typedResponse = response as WazuhResponse;
        expect(typedResponse.data).toBeDefined();
        
        appendToDoc('Packages Info', typedResponse);
    }, 30000);

    afterAll(() => {
        // Write documentation to file
        const docsDir = path.join(__dirname, '..', 'docs');
        
        if (!fs.existsSync(docsDir)) {
            fs.mkdirSync(docsDir, { recursive: true });
        }
        
        fs.writeFileSync(
            path.join(docsDir, 'wazuh-syscollector-responses.md'),
            documentation
        );
    });
});
