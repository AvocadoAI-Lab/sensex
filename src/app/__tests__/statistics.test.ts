import { makeAuthorizedRequest } from '../utils/auth-helper';

describe('Wazuh Statistics API Flow', () => {
    // Create documentation
    let documentation = '# Wazuh Statistics API Test Results\n\n';
    const appendToDoc = (section: string, content: any) => {
        documentation += `## ${section}\n\`\`\`json\n${JSON.stringify(content, null, 2)}\n\`\`\`\n\n`;
    };

    test('should get hourly statistics', async () => {
        const response = await makeAuthorizedRequest('/statistics/hourly');
        
        expect(response).toBeDefined();
        expect(response.data).toBeDefined();
        
        appendToDoc('Hourly Statistics', response);
    }, 30000);

    test('should get weekly statistics', async () => {
        const response = await makeAuthorizedRequest('/statistics/weekly');
        
        expect(response).toBeDefined();
        expect(response.data).toBeDefined();
        
        appendToDoc('Weekly Statistics', response);
    }, 30000);

    test('should get analysisd statistics', async () => {
        const response = await makeAuthorizedRequest('/statistics/analysisd');
        
        expect(response).toBeDefined();
        expect(response.data).toBeDefined();
        
        appendToDoc('Analysisd Statistics', {
            description: 'Statistics from the analysis daemon (analysisd)',
            response: response
        });
    }, 30000);

    test('should get remoted statistics', async () => {
        const response = await makeAuthorizedRequest('/statistics/remoted');
        
        expect(response).toBeDefined();
        expect(response.data).toBeDefined();
        
        appendToDoc('Remoted Statistics', {
            description: 'Statistics from the remote daemon (remoted)',
            response: response
        });
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
            path.join(docsDir, 'wazuh-statistics-responses.md'),
            documentation
        );
    });
});
