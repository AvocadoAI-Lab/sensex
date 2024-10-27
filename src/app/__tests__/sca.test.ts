import { makeAuthorizedRequest } from '../utils/auth-helper';

describe('Wazuh SCA (Security Configuration Assessment) API Flow', () => {
    // Create documentation
    let documentation = '# Wazuh SCA API Test Results\n\n';
    const appendToDoc = (section: string, content: any) => {
        documentation += `## ${section}\n\`\`\`json\n${JSON.stringify(content, null, 2)}\n\`\`\`\n\n`;
    };

    let agentId: string;

    beforeAll(async () => {
        // Get first available agent ID
        const response = await makeAuthorizedRequest('/agents');
        
        if (response.data.affected_items.length > 0) {
            agentId = response.data.affected_items[0].id;
            console.log('Using agent ID:', agentId);
            appendToDoc('Selected Agent', { id: agentId });
        } else {
            console.log('No agents available for testing');
        }
    }, 30000);

    test('should get SCA policies for agent', async () => {
        if (!agentId) {
            console.log('No agent available, skipping SCA policies test');
            appendToDoc('SCA Policies', { message: 'Test skipped - No agent available' });
            return;
        }

        // Get agent's SCA policies
        const response = await makeAuthorizedRequest(`/sca/${agentId}/policies`);
        
        expect(response).toBeDefined();
        expect(response.data).toBeDefined();
        
        appendToDoc('SCA Policies', response);
    }, 30000);

    test('should get SCA checks for policy', async () => {
        if (!agentId) {
            console.log('No agent available, skipping SCA checks test');
            appendToDoc('SCA Checks', { message: 'Test skipped - No agent available' });
            return;
        }

        // First get available policies
        const policiesResponse = await makeAuthorizedRequest(`/sca/${agentId}/policies`);
        
        if (policiesResponse.data.affected_items && policiesResponse.data.affected_items.length > 0) {
            const policyId = policiesResponse.data.affected_items[0].policy_id;
            
            // Get checks for the first policy
            const response = await makeAuthorizedRequest(`/sca/${agentId}/checks/${policyId}`);
            
            expect(response).toBeDefined();
            expect(response.data).toBeDefined();
            
            appendToDoc('SCA Checks', response);
        } else {
            console.log('No policies available for SCA checks test');
            appendToDoc('SCA Checks', { message: 'Test skipped - No policies available' });
        }
    }, 30000);

    test('should get SCA scan results', async () => {
        if (!agentId) {
            console.log('No agent available, skipping SCA scan results test');
            appendToDoc('SCA Scan Results', { message: 'Test skipped - No agent available' });
            return;
        }

        // Get agent's SCA scan results
        const response = await makeAuthorizedRequest(`/sca/${agentId}/results`);
        
        expect(response).toBeDefined();
        expect(response.data).toBeDefined();
        
        appendToDoc('SCA Scan Results', response);
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
            path.join(docsDir, 'wazuh-sca-responses.md'),
            documentation
        );
    });
});
