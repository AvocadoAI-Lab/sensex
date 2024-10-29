import { makeAuthorizedRequest } from '../utils/auth-helper';
import fs from 'fs';
import path from 'path';
import type { WazuhResponse } from '../types/responses';

describe('Wazuh Decoders API Flow', () => {
    // Create documentation
    let documentation = '# Wazuh Decoders API Test Results\n\n';
    const appendToDoc = (section: string, content: WazuhResponse): void => {
        documentation += `## ${section}\n\`\`\`json\n${JSON.stringify(content, null, 2)}\n\`\`\`\n\n`;
    };

    test('should get decoders list', async () => {
        const response = await makeAuthorizedRequest('/decoders');
        
        expect(response).toBeDefined();
        if (!response) {
            throw new Error('Response is null');
        }

        const typedResponse = response as WazuhResponse;
        expect(typedResponse.data).toBeDefined();
        expect(typedResponse.data.affected_items).toBeDefined();
        expect(Array.isArray(typedResponse.data.affected_items)).toBe(true);
        
        appendToDoc('Decoders List', typedResponse);
    }, 30000);

    afterAll(() => {
        // Write documentation to file
        const docsDir = path.join(__dirname, '..', 'docs');
        
        if (!fs.existsSync(docsDir)) {
            fs.mkdirSync(docsDir, { recursive: true });
        }
        
        fs.writeFileSync(
            path.join(docsDir, 'wazuh-decoders-responses.md'),
            documentation
        );
    });
});
