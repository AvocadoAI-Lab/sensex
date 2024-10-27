import { makeAuthorizedRequest } from '../utils/auth-helper';

describe('Wazuh Lists API Flow', () => {
    // Create documentation
    let documentation = '# Wazuh Lists API Test Results\n\n';
    const appendToDoc = (section: string, content: any) => {
        documentation += `## ${section}\n\`\`\`json\n${JSON.stringify(content, null, 2)}\n\`\`\`\n\n`;
    };

    test('should get CDB lists files', async () => {
        const response = await makeAuthorizedRequest('/lists/files');
        
        expect(response).toBeDefined();
        expect(response.data).toBeDefined();
        expect(response.data.affected_items).toBeDefined();
        expect(Array.isArray(response.data.affected_items)).toBe(true);
        
        appendToDoc('CDB Lists Files', response);

        // If we have any list files, test getting content of the first one
        if (response.data.affected_items && response.data.affected_items.length > 0) {
            const filename = response.data.affected_items[0].filename;
            console.log('Getting content for list:', filename);

            const contentResponse = await makeAuthorizedRequest(`/lists/files/${filename}`);
            
            expect(contentResponse).toBeDefined();
            expect(contentResponse.data).toBeDefined();
            
            appendToDoc(`List Content (${filename})`, contentResponse);
        } else {
            console.log('No list files available');
            appendToDoc('List Content', { message: 'No list files available to test content retrieval' });
        }
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
            path.join(docsDir, 'wazuh-lists-responses.md'),
            documentation
        );
    });
});
