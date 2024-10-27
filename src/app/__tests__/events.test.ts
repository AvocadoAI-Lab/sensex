import { makeAuthorizedRequest } from '../utils/auth-helper';

describe('Wazuh Events API Flow', () => {
    // Create documentation
    let documentation = '# Wazuh Events API Test Results\n\n';
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

    test('should get events list for agent', async () => {
        if (!agentId) {
            console.log('No agent available, skipping events list test');
            appendToDoc('Events List', { message: 'Test skipped - No agent available' });
            return;
        }

        const response = await makeAuthorizedRequest(`/syscheck/${agentId}/last_scan`);
        
        expect(response).toBeDefined();
        expect(response.data).toBeDefined();
        
        appendToDoc('Events List', response);
    }, 30000);

    test('should get events summary', async () => {
        const response = await makeAuthorizedRequest('/overview/events');
        
        expect(response).toBeDefined();
        expect(response.data).toBeDefined();
        
        appendToDoc('Events Summary', response);
    }, 30000);

    test('should get events alerts', async () => {
        const response = await makeAuthorizedRequest('/alerts');
        
        expect(response).toBeDefined();
        expect(response.data).toBeDefined();
        
        appendToDoc('Events Alerts', response);
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
            path.join(docsDir, 'wazuh-events-responses.md'),
            documentation
        );
    });
});
