import { makeAuthorizedRequest } from '../utils/auth-helper';

describe('Wazuh CIS-CAT API Flow', () => {
    // Create documentation
    let documentation = '# Wazuh CIS-CAT API Test Results\n\n';
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

    test('should get CIS-CAT results for agent', async () => {
        if (!agentId) {
            console.log('No agent available, skipping CIS-CAT results test');
            appendToDoc('CIS-CAT Results', { message: 'Test skipped - No agent available' });
            return;
        }

        const response = await makeAuthorizedRequest(`/ciscat/${agentId}/results`);
        
        expect(response).toBeDefined();
        expect(response.data).toBeDefined();
        
        appendToDoc('CIS-CAT Results', response);
    }, 30000);

    test('should get CIS-CAT scan info for agent', async () => {
        if (!agentId) {
            console.log('No agent available, skipping CIS-CAT scan info test');
            appendToDoc('CIS-CAT Scan Info', { message: 'Test skipped - No agent available' });
            return;
        }

        const response = await makeAuthorizedRequest(`/ciscat/${agentId}/scan_info`);
        
        expect(response).toBeDefined();
        expect(response.data).toBeDefined();
        
        appendToDoc('CIS-CAT Scan Info', response);
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
            path.join(docsDir, 'wazuh-ciscat-responses.md'),
            documentation
        );
    });
});
