import { makeAuthorizedRequest } from '../utils/auth-helper';

describe('Wazuh Syscollector API Flow', () => {
    // Create documentation
    let documentation = '# Wazuh Syscollector API Test Results\n\n';
    const appendToDoc = (section: string, content: any) => {
        documentation += `## ${section}\n\`\`\`json\n${JSON.stringify(content, null, 2)}\n\`\`\`\n\n`;
    };

    let agentId: string;

    beforeAll(async () => {
        // Get first available agent ID
        const response = await makeAuthorizedRequest('/agents');
        
        if (response.data.affected_items.length > 0) {
            agentId = response.data.affected_items[0].id;
            console.log('Using agent ID:', agentId);
            appendToDoc('Selected Agent', { id: agentId });
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

        const response = await makeAuthorizedRequest(`/syscollector/${agentId}/hardware`);
        
        expect(response).toBeDefined();
        expect(response.data).toBeDefined();
        
        appendToDoc('Hardware Info', response);
    }, 30000);

    test('should get OS info for agent', async () => {
        if (!agentId) {
            console.log('No agent available, skipping OS test');
            appendToDoc('OS Info', { message: 'Test skipped - No agent available' });
            return;
        }

        const response = await makeAuthorizedRequest(`/syscollector/${agentId}/os`);
        
        expect(response).toBeDefined();
        expect(response.data).toBeDefined();
        
        appendToDoc('OS Info', response);
    }, 30000);

    test('should get packages info for agent', async () => {
        if (!agentId) {
            console.log('No agent available, skipping packages test');
            appendToDoc('Packages Info', { message: 'Test skipped - No agent available' });
            return;
        }

        const response = await makeAuthorizedRequest(`/syscollector/${agentId}/packages`);
        
        expect(response).toBeDefined();
        expect(response.data).toBeDefined();
        
        appendToDoc('Packages Info', response);
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
            path.join(docsDir, 'wazuh-syscollector-responses.md'),
            documentation
        );
    });
});
