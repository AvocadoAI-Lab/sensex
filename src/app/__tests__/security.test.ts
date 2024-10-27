import { makeAuthorizedRequest } from '../utils/auth-helper';

describe('Wazuh Security API Flow', () => {
    // Create documentation
    let documentation = '# Wazuh Security API Test Results\n\n';
    const appendToDoc = (section: string, content: any) => {
        documentation += `## ${section}\n\`\`\`json\n${JSON.stringify(content, null, 2)}\n\`\`\`\n\n`;
    };

    test('should get security users', async () => {
        const response = await makeAuthorizedRequest('/security/users');
        
        expect(response).toBeDefined();
        expect(response.data).toBeDefined();
        expect(response.data.affected_items).toBeDefined();
        expect(Array.isArray(response.data.affected_items)).toBe(true);
        
        appendToDoc('Security Users', response);
    }, 30000);

    test('should get security roles', async () => {
        const response = await makeAuthorizedRequest('/security/roles');
        
        expect(response).toBeDefined();
        expect(response.data).toBeDefined();
        expect(response.data.affected_items).toBeDefined();
        expect(Array.isArray(response.data.affected_items)).toBe(true);
        
        appendToDoc('Security Roles', response);
    }, 30000);

    test('should get security policies', async () => {
        const response = await makeAuthorizedRequest('/security/policies');
        
        expect(response).toBeDefined();
        expect(response.data).toBeDefined();
        expect(response.data.affected_items).toBeDefined();
        expect(Array.isArray(response.data.affected_items)).toBe(true);
        
        appendToDoc('Security Policies', response);
    }, 30000);

    test('should get security rules', async () => {
        const response = await makeAuthorizedRequest('/security/rules');
        
        expect(response).toBeDefined();
        expect(response.data).toBeDefined();
        expect(response.data.affected_items).toBeDefined();
        expect(Array.isArray(response.data.affected_items)).toBe(true);
        
        appendToDoc('Security Rules', response);
    }, 30000);

    test('should get security resources', async () => {
        const response = await makeAuthorizedRequest('/security/resources');
        
        expect(response).toBeDefined();
        expect(response.data).toBeDefined();
        expect(response.data.affected_items).toBeDefined();
        expect(Array.isArray(response.data.affected_items)).toBe(true);
        
        appendToDoc('Security Resources', response);
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
            path.join(docsDir, 'wazuh-security-responses.md'),
            documentation
        );
    });
});
