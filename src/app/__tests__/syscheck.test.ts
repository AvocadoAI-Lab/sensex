import { makeAuthorizedRequest } from '../utils/auth-helper';
import fs from 'fs';
import path from 'path';
import type { WazuhResponse } from '../types/responses';

describe('Wazuh Syscheck API Flow', () => {
    // Create documentation
    let documentation = '# Wazuh Syscheck API Test Results\n\n';
    const appendToDoc = (section: string, content: WazuhResponse | { message: string; id?: string }): void => {
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
            if (!firstAgent || typeof firstAgent.id !== 'string') {
                throw new Error('Invalid agent data structure');
            }
            // Get the first agent ID and ensure it's at least 3 digits
            agentId = firstAgent.id.padStart(3, '0');
            console.log('Using agent ID:', agentId);
            appendToDoc('Selected Agent', { id: agentId, message: 'Agent selected for testing' });
        } else {
            console.log('No agents available for testing');
        }
    });

    test('should get syscheck files for agent', async () => {
        if (!agentId) {
            console.log('No agent available, skipping test');
            appendToDoc('Syscheck Files', { message: 'Test skipped - No agent available' });
            return;
        }

        const response = await makeAuthorizedRequest(`/syscheck/${agentId}`, 'POST', {
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
        
        appendToDoc('Syscheck Files', typedResponse);
    }, 30000);

    test('should get syscheck last scan info', async () => {
        if (!agentId) {
            console.log('No agent available, skipping test');
            appendToDoc('Last Scan Info', { message: 'Test skipped - No agent available' });
            return;
        }

        const response = await makeAuthorizedRequest(`/syscheck/${agentId}/last_scan`, 'POST', {
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
        
        appendToDoc('Last Scan Info', typedResponse);
    }, 30000);

    afterAll(() => {
        // Write documentation to file
        const docsDir = path.join(__dirname, '..', 'docs');
        
        if (!fs.existsSync(docsDir)) {
            fs.mkdirSync(docsDir, { recursive: true });
        }
        
        fs.writeFileSync(
            path.join(docsDir, 'wazuh-syscheck-responses.md'),
            documentation
        );
    });
});
