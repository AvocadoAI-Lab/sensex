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
    });

    test('should get SCA policies for agent', async () => {
        if (!agentId) {
            console.log('No agent available, skipping SCA policies test');
            appendToDoc('SCA Policies', { message: 'Test skipped - No agent available' });
            return;
        }

        const response = await makeAuthorizedRequest(`/sca/${agentId}/policies`);
        
        expect(response).toBeDefined();
        expect(response.data).toBeDefined();
        
        appendToDoc('SCA Policies', response);

        // If we get policies, let's also test getting checks and results for the first policy
        if (response.data.affected_items && response.data.affected_items.length > 0) {
            const policyId = response.data.affected_items[0].policy_id;
            
            // Get checks for this policy
            const checksResponse = await makeAuthorizedRequest(`/sca/${agentId}/checks/${policyId}`);
            expect(checksResponse).toBeDefined();
            appendToDoc(`SCA Checks for Policy ${policyId}`, checksResponse);

            // Get results for this policy
            const resultsResponse = await makeAuthorizedRequest(`/sca/${agentId}/results/${policyId}`);
            expect(resultsResponse).toBeDefined();
            appendToDoc(`SCA Results for Policy ${policyId}`, resultsResponse);
        }
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
