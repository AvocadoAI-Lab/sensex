import { makeAuthorizedRequest } from '../utils/auth-helper';

describe('Wazuh Rootcheck API Flow', () => {
    // Create documentation
    let documentation = '# Wazuh Rootcheck API Test Results\n\n';
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

    test('should get rootcheck database for agent', async () => {
        if (!agentId) {
            console.log('No agent available, skipping rootcheck database test');
            appendToDoc('Rootcheck Database', { message: 'Test skipped - No agent available' });
            return;
        }

        const response = await makeAuthorizedRequest(`/rootcheck/${agentId}`);
        
        expect(response).toBeDefined();
        expect(response.data).toBeDefined();
        
        appendToDoc('Rootcheck Database', response);
    }, 30000);

    test('should get rootcheck last scan for agent', async () => {
        if (!agentId) {
            console.log('No agent available, skipping rootcheck last scan test');
            appendToDoc('Rootcheck Last Scan', { message: 'Test skipped - No agent available' });
            return;
        }

        const response = await makeAuthorizedRequest(`/rootcheck/${agentId}/last_scan`);
        
        expect(response).toBeDefined();
        expect(response.data).toBeDefined();
        
        appendToDoc('Rootcheck Last Scan', response);
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
            path.join(docsDir, 'wazuh-rootcheck-responses.md'),
            documentation
        );
    });
});
