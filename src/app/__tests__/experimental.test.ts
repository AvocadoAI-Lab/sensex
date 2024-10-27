import { makeAuthorizedRequest } from '../utils/auth-helper';

describe('Wazuh Experimental API Flow', () => {
    // Create documentation
    let documentation = '# Wazuh Experimental API Test Results\n\n';
    const appendToDoc = (section: string, content: any) => {
        documentation += `## ${section}\n\`\`\`json\n${JSON.stringify(content, null, 2)}\n\`\`\`\n\n`;
    };

    let experimentalEnabled = true;

    beforeAll(async () => {
        try {
            await makeAuthorizedRequest('/experimental/ciscat/results');
        } catch (err) {
            const error = err as Error;
            if (error && error.message && error.message.includes('Experimental features are disabled')) {
                console.log('Experimental features are disabled, skipping tests');
                experimentalEnabled = false;
            }
        }
    }, 30000);

    test('should get experimental CIS-CAT results', async () => {
        if (!experimentalEnabled) {
            console.log('Skipping experimental CIS-CAT test - features disabled');
            appendToDoc('Experimental CIS-CAT Results', { 
                message: 'Test skipped - Experimental features disabled'
            });
            return;
        }

        const response = await makeAuthorizedRequest('/experimental/ciscat/results');
        
        expect(response).toBeDefined();
        expect(response.data).toBeDefined();
        
        appendToDoc('Experimental CIS-CAT Results', response);
    }, 30000);

    test('should test experimental rules', async () => {
        if (!experimentalEnabled) {
            console.log('Skipping experimental rules test - features disabled');
            appendToDoc('Experimental Rules Test', { 
                message: 'Test skipped - Experimental features disabled'
            });
            return;
        }

        const testRule = {
            decoder: {
                name: "test_decoder",
                parent: "json",
                program_name: "test_program"
            },
            location: "test_location",
            log_format: "test_format",
            log_string: "test string"
        };

        const response = await makeAuthorizedRequest(
            '/experimental/rules/test',
            'PUT',
            testRule
        );
        
        expect(response).toBeDefined();
        expect(response.data).toBeDefined();
        
        appendToDoc('Experimental Rules Test', {
            request: testRule,
            response: response
        });
    }, 30000);

    test('should test experimental decoders', async () => {
        if (!experimentalEnabled) {
            console.log('Skipping experimental decoders test - features disabled');
            appendToDoc('Experimental Decoders Test', { 
                message: 'Test skipped - Experimental features disabled'
            });
            return;
        }

        const testDecoder = {
            decoder: {
                name: "test_decoder",
                parent: "json",
                program_name: "test_program"
            },
            location: "test_location",
            log_format: "test_format",
            log_string: "test string"
        };

        const response = await makeAuthorizedRequest(
            '/experimental/decoders/test',
            'PUT',
            testDecoder
        );
        
        expect(response).toBeDefined();
        expect(response.data).toBeDefined();
        
        appendToDoc('Experimental Decoders Test', {
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
