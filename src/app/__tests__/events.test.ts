import { makeAuthorizedRequest } from '../utils/auth-helper';

describe('Wazuh Events API Flow', () => {
    // Create documentation
    let documentation = '# Wazuh Events API Test Results\n\n';
    const appendToDoc = (section: string, content: any) => {
        documentation += `## ${section}\n\`\`\`json\n${JSON.stringify(content, null, 2)}\n\`\`\`\n\n`;
    };

    test('should get events summary', async () => {
        const response = await makeAuthorizedRequest('/manager/stats');
        
        expect(response).toBeDefined();
        expect(response.data).toBeDefined();
        expect(response.data.affected_items).toBeDefined();
        expect(Array.isArray(response.data.affected_items)).toBe(true);
        
        appendToDoc('Events Summary', response);
    }, 30000);

    test('should get events alerts', async () => {
        const response = await makeAuthorizedRequest('/manager/logs');
        
        expect(response).toBeDefined();
        expect(response.data).toBeDefined();
        expect(response.data.affected_items).toBeDefined();
        expect(Array.isArray(response.data.affected_items)).toBe(true);
        
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
