import { makeAuthorizedRequest } from '../utils/auth-helper';

describe('Wazuh Experimental API Flow', () => {
    // Create documentation
    let documentation = '# Wazuh Experimental API Test Results\n\n';
    const appendToDoc = (section: string, content: any) => {
        documentation += `## ${section}\n\`\`\`json\n${JSON.stringify(content, null, 2)}\n\`\`\`\n\n`;
    };

    test('should get experimental CIS-CAT results', async () => {
        const response = await makeAuthorizedRequest('/experimental/ciscat/results');
        
        expect(response).toBeDefined();
        expect(response.data).toBeDefined();
        
        appendToDoc('Experimental CIS-CAT Results', {
            description: 'Results from experimental CIS-CAT scan',
            response: response
        });
    }, 30000);

    test('should test experimental rules', async () => {
        const testRule = {
            rule: {
                level: 5,
                description: "Test experimental rule",
                id: "100001",
                pattern: "test pattern"
            }
        };

        const response = await makeAuthorizedRequest(
            '/experimental/rules',
            'POST',
            testRule
        );
        
        expect(response).toBeDefined();
        expect(response.data).toBeDefined();
        
        appendToDoc('Experimental Rules Test', {
            description: 'Testing experimental rule creation',
            request: testRule,
            response: response
        });
    }, 30000);

    test('should test experimental decoders', async () => {
        const testDecoder = {
            decoder: {
                name: "test_decoder",
                program_name: "test_program",
                pattern: "test pattern"
            }
        };

        const response = await makeAuthorizedRequest(
            '/experimental/decoders',
            'POST',
            testDecoder
        );
        
        expect(response).toBeDefined();
        expect(response.data).toBeDefined();
        
        appendToDoc('Experimental Decoders Test', {
            description: 'Testing experimental decoder creation',
            request: testDecoder,
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
            path.join(docsDir, 'wazuh-experimental-responses.md'),
            documentation
        );
    });
});
