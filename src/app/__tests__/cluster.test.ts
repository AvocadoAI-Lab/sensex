import { makeAuthorizedRequest } from '../utils/auth-helper';

describe('Wazuh Cluster API Flow', () => {
    // Create documentation
    let documentation = '# Wazuh Cluster API Test Results\n\n';
    const appendToDoc = (section: string, content: any) => {
        documentation += `## ${section}\n\`\`\`json\n${JSON.stringify(content, null, 2)}\n\`\`\`\n\n`;
    };

    let isClusterEnabled: boolean = false;

    test('should get cluster status', async () => {
        const response = await makeAuthorizedRequest('/cluster/status');
        
        expect(response).toBeDefined();
        expect(response.data).toBeDefined();
        
        appendToDoc('Cluster Status', response);
        
        // Store cluster status for subsequent tests
        isClusterEnabled = response.data.enabled === 'yes';
        console.log('Cluster enabled:', isClusterEnabled);
    }, 30000);

    test('should get cluster nodes if enabled', async () => {
        if (!isClusterEnabled) {
            console.log('Cluster is disabled, skipping nodes test');
            appendToDoc('Cluster Nodes', { message: 'Test skipped - Cluster is disabled' });
            return;
        }

        const response = await makeAuthorizedRequest('/cluster/nodes');
        
        expect(response).toBeDefined();
        expect(response.data).toBeDefined();
        
        appendToDoc('Cluster Nodes', response);
    }, 30000);

    test('should get cluster health if enabled', async () => {
        if (!isClusterEnabled) {
            console.log('Cluster is disabled, skipping health test');
            appendToDoc('Cluster Health', { message: 'Test skipped - Cluster is disabled' });
            return;
        }

        const response = await makeAuthorizedRequest('/cluster/healthcheck');
        
        expect(response).toBeDefined();
        expect(response.data).toBeDefined();
        
        appendToDoc('Cluster Health', response);
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
            path.join(docsDir, 'wazuh-cluster-responses.md'),
            documentation
        );
    });
});
