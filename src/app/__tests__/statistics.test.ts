import { makeAuthorizedRequest } from '../utils/auth-helper';

describe('Wazuh Statistics API Flow', () => {
    // Create documentation
    let documentation = '# Wazuh Statistics API Test Results\n\n';
    const appendToDoc = (section: string, content: any) => {
        documentation += `## ${section}\n\`\`\`json\n${JSON.stringify(content, null, 2)}\n\`\`\`\n\n`;
    };

    test('should get manager stats', async () => {
        const response = await makeAuthorizedRequest('/manager/stats');
        
        expect(response).toBeDefined();
        expect(response.data).toBeDefined();
        
        appendToDoc('Manager Statistics', response);
    }, 30000);

    test('should get manager stats by hour', async () => {
        const response = await makeAuthorizedRequest('/manager/stats/hourly');
        
        expect(response).toBeDefined();
        expect(response.data).toBeDefined();
        
        appendToDoc('Hourly Statistics', response);
    }, 30000);

    test('should get manager stats by week', async () => {
        const response = await makeAuthorizedRequest('/manager/stats/weekly');
        
        expect(response).toBeDefined();
        expect(response.data).toBeDefined();
        
        appendToDoc('Weekly Statistics', response);
    }, 30000);

    test('should get manager stats by analyzer', async () => {
        const response = await makeAuthorizedRequest('/manager/stats/analysisd');
        
        expect(response).toBeDefined();
        expect(response.data).toBeDefined();
        
        appendToDoc('Analyzer Statistics', response);
    }, 30000);

    test('should get manager stats by remote', async () => {
        const response = await makeAuthorizedRequest('/manager/stats/remoted');
        
        expect(response).toBeDefined();
        expect(response.data).toBeDefined();
        
        appendToDoc('Remote Statistics', response);
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
