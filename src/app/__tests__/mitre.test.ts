import { makeAuthorizedRequest } from '../utils/auth-helper';

describe('Wazuh MITRE API Flow', () => {
    // Create documentation
    let documentation = '# Wazuh MITRE API Test Results\n\n';
    const appendToDoc = (section: string, content: any) => {
        documentation += `## ${section}\n\`\`\`json\n${JSON.stringify(content, null, 2)}\n\`\`\`\n\n`;
    };

    test('should get MITRE metadata', async () => {
        const response = await makeAuthorizedRequest('/mitre/metadata', 'POST');
        
        expect(response).toBeDefined();
        expect(response.data).toBeDefined();
        
        appendToDoc('MITRE Metadata', response);
    }, 30000);

    test('should get MITRE tactics', async () => {
        const response = await makeAuthorizedRequest('/mitre/tactics', 'POST');
        
        expect(response).toBeDefined();
        expect(response.data).toBeDefined();
        
        appendToDoc('MITRE Tactics', response);
    }, 30000);

    test('should get MITRE techniques', async () => {
        const response = await makeAuthorizedRequest('/mitre/techniques', 'POST');
        
        expect(response).toBeDefined();
        expect(response.data).toBeDefined();
        
        appendToDoc('MITRE Techniques', response);
    }, 30000);

    test('should get MITRE references', async () => {
        const response = await makeAuthorizedRequest('/mitre/references', 'POST');
        
        expect(response).toBeDefined();
        expect(response.data).toBeDefined();
        
        appendToDoc('MITRE References', response);
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
            path.join(docsDir, 'wazuh-mitre-responses.md'),
            documentation
        );
    });
});
