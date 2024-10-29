import { makeAuthorizedRequest } from '../utils/auth-helper';
import fs from 'fs';
import path from 'path';
import type { WazuhResponse } from '../types/responses';

describe('Wazuh MITRE API Flow', () => {
    // Create documentation
    let documentation = '# Wazuh MITRE API Test Results\n\n';
    const appendToDoc = (section: string, content: WazuhResponse): void => {
        documentation += `## ${section}\n\`\`\`json\n${JSON.stringify(content, null, 2)}\n\`\`\`\n\n`;
    };

    test('should get MITRE metadata', async () => {
        const response = await makeAuthorizedRequest('/mitre/metadata', 'POST');
        
        expect(response).toBeDefined();
        if (!response) {
            throw new Error('Response is null');
        }

        const typedResponse = response as WazuhResponse;
        expect(typedResponse.data).toBeDefined();
        
        appendToDoc('MITRE Metadata', typedResponse);
    }, 30000);

    test('should get MITRE tactics', async () => {
        const response = await makeAuthorizedRequest('/mitre/tactics', 'POST');
        
        expect(response).toBeDefined();
        if (!response) {
            throw new Error('Response is null');
        }

        const typedResponse = response as WazuhResponse;
        expect(typedResponse.data).toBeDefined();
        
        appendToDoc('MITRE Tactics', typedResponse);
    }, 30000);

    test('should get MITRE techniques', async () => {
        const response = await makeAuthorizedRequest('/mitre/techniques', 'POST');
        
        expect(response).toBeDefined();
        if (!response) {
            throw new Error('Response is null');
        }

        const typedResponse = response as WazuhResponse;
        expect(typedResponse.data).toBeDefined();
        
        appendToDoc('MITRE Techniques', typedResponse);
    }, 30000);

    test('should get MITRE references', async () => {
        const response = await makeAuthorizedRequest('/mitre/references', 'POST');
        
        expect(response).toBeDefined();
        if (!response) {
            throw new Error('Response is null');
        }

        const typedResponse = response as WazuhResponse;
        expect(typedResponse.data).toBeDefined();
        
        appendToDoc('MITRE References', typedResponse);
    }, 30000);

    afterAll(() => {
        // Write documentation to file
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
