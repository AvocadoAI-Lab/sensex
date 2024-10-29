import { makeAuthorizedRequest } from '../utils/auth-helper';
import fs from 'fs';
import path from 'path';
import type { WazuhResponse } from '../types/responses';

describe('Wazuh Manager API Flow', () => {
    // Create documentation
    let documentation = '# Wazuh Manager API Test Results\n\n';
    const appendToDoc = (section: string, content: WazuhResponse): void => {
        documentation += `## ${section}\n\`\`\`json\n${JSON.stringify(content, null, 2)}\n\`\`\`\n\n`;
    };

    test('should get manager info', async () => {
        const response = await makeAuthorizedRequest('/manager/info');
        
        expect(response).toBeDefined();
        if (!response) {
            throw new Error('Response is null');
        }

        const typedResponse = response as WazuhResponse;
        expect(typedResponse.data).toBeDefined();
        
        appendToDoc('Manager Info', typedResponse);
    }, 30000);

    test('should get manager status', async () => {
        const response = await makeAuthorizedRequest('/manager/status');
        
        expect(response).toBeDefined();
        if (!response) {
            throw new Error('Response is null');
        }

        const typedResponse = response as WazuhResponse;
        expect(typedResponse.data).toBeDefined();
        
        appendToDoc('Manager Status', typedResponse);
    }, 30000);

    test('should get manager logs', async () => {
        const response = await makeAuthorizedRequest('/manager/logs');
        
        expect(response).toBeDefined();
        if (!response) {
            throw new Error('Response is null');
        }

        const typedResponse = response as WazuhResponse;
        expect(typedResponse.data).toBeDefined();
        
        appendToDoc('Manager Logs', typedResponse);
    }, 30000);

    afterAll(() => {
        // Write documentation to file
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
