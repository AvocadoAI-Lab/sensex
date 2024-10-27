import { makeAuthorizedRequest } from '../utils/auth-helper';

describe('Wazuh Rules API Flow', () => {
    // Create documentation
    let documentation = '# Wazuh Rules API Test Results\n\n';
    const appendToDoc = (section: string, content: any) => {
        documentation += `## ${section}\n\`\`\`json\n${JSON.stringify(content, null, 2)}\n\`\`\`\n\n`;
    };

    test('should get rules list', async () => {
        const response = await makeAuthorizedRequest('/rules');
        
        expect(response).toBeDefined();
        expect(response.data).toBeDefined();
        expect(response.data.affected_items).toBeDefined();
        expect(Array.isArray(response.data.affected_items)).toBe(true);
        
        appendToDoc('Rules List', response);
    }, 30000);

    test('should get rules groups', async () => {
        const response = await makeAuthorizedRequest('/rules/groups');
        
        expect(response).toBeDefined();
        expect(response.data).toBeDefined();
        expect(response.data.affected_items).toBeDefined();
        expect(Array.isArray(response.data.affected_items)).toBe(true);
        
        appendToDoc('Rules Groups', response);
    }, 30000);

    test('should get rules files', async () => {
        const response = await makeAuthorizedRequest('/rules/files');
        
        expect(response).toBeDefined();
        expect(response.data).toBeDefined();
        expect(response.data.affected_items).toBeDefined();
        expect(Array.isArray(response.data.affected_items)).toBe(true);
        
        appendToDoc('Rules Files', response);
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
            path.join(docsDir, 'wazuh-rules-responses.md'),
            documentation
        );
    });
});
