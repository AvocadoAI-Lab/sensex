import { makeAuthorizedRequest } from '../utils/auth-helper';

describe('Wazuh Tasks API Flow', () => {
    // Create documentation
    let documentation = '# Wazuh Tasks API Test Results\n\n';
    const appendToDoc = (section: string, content: any) => {
        documentation += `## ${section}\n\`\`\`json\n${JSON.stringify(content, null, 2)}\n\`\`\`\n\n`;
    };

    test('should get tasks list', async () => {
        const response = await makeAuthorizedRequest('/tasks');
        
        expect(response).toBeDefined();
        expect(response.data).toBeDefined();
        
        appendToDoc('Tasks List', response);
    }, 30000);

    test('should get tasks status', async () => {
        const response = await makeAuthorizedRequest('/tasks/status');
        
        expect(response).toBeDefined();
        expect(response.data).toBeDefined();
        
        appendToDoc('Tasks Status', response);
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
            path.join(docsDir, 'wazuh-tasks-responses.md'),
            documentation
        );
    });
});
