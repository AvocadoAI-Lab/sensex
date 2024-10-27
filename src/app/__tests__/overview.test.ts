import { makeAuthorizedRequest } from '../utils/auth-helper';

describe('Wazuh Overview API Flow', () => {
    // Create documentation
    let documentation = '# Wazuh Overview API Test Results\n\n';
    const appendToDoc = (section: string, content: any) => {
        documentation += `## ${section}\n\`\`\`json\n${JSON.stringify(content, null, 2)}\n\`\`\`\n\n`;
    };

    test('should get agents summary', async () => {
        const response = await makeAuthorizedRequest('/agents/summary/status');
        
        expect(response).toBeDefined();
        expect(response.data).toBeDefined();
        
        appendToDoc('Agents Summary', response);
    }, 30000);

    test('should get security events summary', async () => {
        const response = await makeAuthorizedRequest('/manager/stats');
        
        expect(response).toBeDefined();
        expect(response.data).toBeDefined();
        
        appendToDoc('Security Events Summary', response);
    }, 30000);

    test('should get file integrity monitoring summary', async () => {
        // Get FIM summary for a specific agent since global summary is not available
        const agentsResponse = await makeAuthorizedRequest('/agents');
        let agentId = '000'; // Default agent
        
        if (agentsResponse.data.affected_items.length > 0) {
            agentId = agentsResponse.data.affected_items[0].id;
        }
        
        const response = await makeAuthorizedRequest(`/syscheck/${agentId}/last_scan`);
        
        expect(response).toBeDefined();
        expect(response.data).toBeDefined();
        
        appendToDoc('File Integrity Monitoring Summary', response);
    }, 30000);

    test('should get vulnerabilities summary', async () => {
        // Get vulnerabilities summary for a specific agent since global summary is not available
        const agentsResponse = await makeAuthorizedRequest('/agents');
        let agentId = '000'; // Default agent
        
        if (agentsResponse.data.affected_items.length > 0) {
            agentId = agentsResponse.data.affected_items[0].id;
        }
        
        const response = await makeAuthorizedRequest(`/vulnerability/${agentId}/summary`);
        
        expect(response).toBeDefined();
        expect(response.data).toBeDefined();
        
        appendToDoc('Vulnerabilities Summary', response);
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
