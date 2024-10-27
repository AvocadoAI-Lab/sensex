import { makeAuthorizedRequest } from '../utils/auth-helper';

describe('Wazuh Syscheck API Flow', () => {
    // Create documentation
    let documentation = '# Wazuh Syscheck API Test Results\n\n';
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
    }, 30000);

    test('should get syscheck files for agent', async () => {
        if (!agentId) {
            console.log('No agent available, skipping syscheck files test');
            appendToDoc('Syscheck Files', { message: 'Test skipped - No agent available' });
            return;
        }

        const response = await makeAuthorizedRequest(`/syscheck/${agentId}`);
        
        expect(response).toBeDefined();
        expect(response.data).toBeDefined();
        
        appendToDoc('Syscheck Files', response);
    }, 30000);

    test('should get syscheck last scan info', async () => {
        if (!agentId) {
            console.log('No agent available, skipping last scan test');
            appendToDoc('Last Scan Info', { message: 'Test skipped - No agent available' });
            return;
        }

        const response = await makeAuthorizedRequest(`/syscheck/${agentId}/last_scan`);
        
        expect(response).toBeDefined();
        expect(response.data).toBeDefined();
        
        appendToDoc('Last Scan Info', response);
    }, 30000);

    test('should get syscheck stats', async () => {
        if (!agentId) {
            console.log('No agent available, skipping syscheck stats test');
            appendToDoc('Syscheck Stats', { message: 'Test skipped - No agent available' });
            return;
        }

        const response = await makeAuthorizedRequest(`/agents/${agentId}/stats/agent`);
        
        expect(response).toBeDefined();
        expect(response.data).toBeDefined();
        
        appendToDoc('Syscheck Stats', {
            description: 'Agent statistics including syscheck information',
            response: response
        });
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
            path.join(docsDir, 'wazuh-syscheck-responses.md'),
            documentation
        );
    });
});
