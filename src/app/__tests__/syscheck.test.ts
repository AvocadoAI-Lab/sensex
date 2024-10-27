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
            // Get the first agent ID and ensure it's at least 3 digits
            agentId = response.data.affected_items[0].id.padStart(3, '0');
            console.log('Using agent ID:', agentId);
            appendToDoc('Selected Agent', { id: agentId });
        } else {
            console.log('No agents available for testing');
        }
    });

    test('should get syscheck files for agent', async () => {
        if (!agentId) {
            console.log('No agent available, skipping test');
            return;
        }

        const response = await makeAuthorizedRequest(`/syscheck/${agentId}`, 'POST', {
            params: {
                agent_id: agentId
            }
        });
        
        expect(response).toBeDefined();
        expect(response.data).toBeDefined();
        
        appendToDoc('Syscheck Files', response);
    }, 30000);

    test('should get syscheck last scan info', async () => {
        if (!agentId) {
            console.log('No agent available, skipping test');
            return;
        }

        const response = await makeAuthorizedRequest(`/syscheck/${agentId}/last_scan`, 'POST', {
            params: {
                agent_id: agentId
            }
        });
        
        expect(response).toBeDefined();
        expect(response.data).toBeDefined();
        
        appendToDoc('Last Scan Info', response);
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
