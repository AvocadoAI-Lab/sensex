import { makeAuthorizedRequest } from '../utils/auth-helper';

describe('Wazuh Decoders API Flow', () => {
    // Create documentation
    let documentation = '# Wazuh Decoders API Test Results\n\n';
    const appendToDoc = (section: string, content: any) => {
        documentation += `## ${section}\n\`\`\`json\n${JSON.stringify(content, null, 2)}\n\`\`\`\n\n`;
    };

    test('should get decoders list', async () => {
        const response = await makeAuthorizedRequest('/decoders');
        
        expect(response).toBeDefined();
        expect(response.data).toBeDefined();
        expect(response.data.affected_items).toBeDefined();
        expect(Array.isArray(response.data.affected_items)).toBe(true);
        
        appendToDoc('Decoders List', response);
    }, 30000);

    test('should get decoders files', async () => {
        const response = await makeAuthorizedRequest('/decoders/files');
        
        expect(response).toBeDefined();
        expect(response.data).toBeDefined();
        expect(response.data.affected_items).toBeDefined();
        expect(Array.isArray(response.data.affected_items)).toBe(true);
        
        appendToDoc('Decoders Files', response);
    }, 30000);

    test('should get decoders parents', async () => {
        const response = await makeAuthorizedRequest('/decoders/parents');
        
        expect(response).toBeDefined();
        expect(response.data).toBeDefined();
        expect(response.data.affected_items).toBeDefined();
        expect(Array.isArray(response.data.affected_items)).toBe(true);
        
        appendToDoc('Decoders Parents', response);
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
            path.join(docsDir, 'wazuh-decoders-responses.md'),
            documentation
        );
    });
});
