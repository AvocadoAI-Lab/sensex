import { makeAuthorizedRequest } from '../utils/auth-helper';

describe('Wazuh Groups API Flow', () => {
    // Create documentation
    let documentation = '# Wazuh Groups API Test Results\n\n';
    const appendToDoc = (section: string, content: any) => {
        documentation += `## ${section}\n\`\`\`json\n${JSON.stringify(content, null, 2)}\n\`\`\`\n\n`;
    };

    let groupId: string;

    test('should get groups list', async () => {
        const response = await makeAuthorizedRequest('/groups');
        
        expect(response).toBeDefined();
        expect(response.data).toBeDefined();
        expect(response.data.affected_items).toBeDefined();
        expect(Array.isArray(response.data.affected_items)).toBe(true);
        
        appendToDoc('Groups List', response);

        // Store first group ID for subsequent tests
        if (response.data.affected_items.length > 0) {
            groupId = response.data.affected_items[0].name;
            console.log('Found group ID:', groupId);
        } else {
            console.log('No groups found');
        }
    }, 30000);

    test('should get group files', async () => {
        if (!groupId) {
            console.log('No group available, skipping group files test');
            appendToDoc('Group Files', { message: 'Test skipped - No group available' });
            return;
        }

        const response = await makeAuthorizedRequest(`/groups/${groupId}/files`);
        
        expect(response).toBeDefined();
        expect(response.data).toBeDefined();
        
        appendToDoc(`Group Files (${groupId})`, response);
    }, 30000);

    test('should get group agents', async () => {
        if (!groupId) {
            console.log('No group available, skipping group agents test');
            appendToDoc('Group Agents', { message: 'Test skipped - No group available' });
            return;
        }

        const response = await makeAuthorizedRequest(`/groups/${groupId}/agents`);
        
        expect(response).toBeDefined();
        expect(response.data).toBeDefined();
        
        appendToDoc(`Group Agents (${groupId})`, response);
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
            path.join(docsDir, 'wazuh-groups-responses.md'),
            documentation
        );
    });
});
