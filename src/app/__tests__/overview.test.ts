import { makeAuthorizedRequest } from '../utils/auth-helper';

describe('Wazuh Overview API Flow', () => {
    // Create documentation
    let documentation = '# Wazuh Overview API Test Results\n\n';
    const appendToDoc = (section: string, content: any) => {
        documentation += `## ${section}\n\`\`\`json\n${JSON.stringify(content, null, 2)}\n\`\`\`\n\n`;
    };

    test('should get agents overview', async () => {
        const response = await makeAuthorizedRequest('/overview/agents');
        
        expect(response).toBeDefined();
        expect(response.data).toBeDefined();
        
        appendToDoc('Agents Overview', response);
    }, 30000);

    test('should get security overview', async () => {
        const response = await makeAuthorizedRequest('/overview/security');
        
        expect(response).toBeDefined();
        expect(response.data).toBeDefined();
        
        appendToDoc('Security Overview', response);
    }, 30000);

    test('should get FIM overview', async () => {
        const response = await makeAuthorizedRequest('/overview/fim');
        
        expect(response).toBeDefined();
        expect(response.data).toBeDefined();
        
        appendToDoc('FIM (File Integrity Monitoring) Overview', response);
    }, 30000);

    test('should get vulnerabilities overview', async () => {
        const response = await makeAuthorizedRequest('/overview/vulnerabilities');
        
        expect(response).toBeDefined();
        expect(response.data).toBeDefined();
        
        appendToDoc('Vulnerabilities Overview', response);
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
            path.join(docsDir, 'wazuh-overview-responses.md'),
            documentation
        );
    });
});
