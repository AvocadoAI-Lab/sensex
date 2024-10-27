import { makeAuthorizedRequest } from '../utils/auth-helper';

describe('Wazuh Manager API Flow', () => {
    // Create documentation
    let documentation = '# Wazuh Manager API Test Results\n\n';
    const appendToDoc = (section: string, content: any) => {
        documentation += `## ${section}\n\`\`\`json\n${JSON.stringify(content, null, 2)}\n\`\`\`\n\n`;
    };

    test('should get manager info', async () => {
        const response = await makeAuthorizedRequest('/manager/info');
        
        expect(response).toBeDefined();
        expect(response.data).toBeDefined();
        
        appendToDoc('Manager Info', response);
    }, 30000);

    test('should get manager status', async () => {
        const response = await makeAuthorizedRequest('/manager/status');
        
        expect(response).toBeDefined();
        expect(response.data).toBeDefined();
        
        appendToDoc('Manager Status', response);
    }, 30000);

    test('should get manager logs', async () => {
        const response = await makeAuthorizedRequest('/manager/logs');
        
        expect(response).toBeDefined();
        expect(response.data).toBeDefined();
        
        appendToDoc('Manager Logs', response);
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
            path.join(docsDir, 'wazuh-manager-responses.md'),
            documentation
        );
    });
});
