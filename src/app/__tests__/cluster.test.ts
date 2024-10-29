import { makeAuthorizedRequest } from '../utils/auth-helper';
import fs from 'fs';
import path from 'path';
import type { WazuhResponse, WazuhAffectedItem } from '../types/responses';

interface ClusterNode extends WazuhAffectedItem {
    name: string;
    type: string;
    version: string;
    ip: string;
    [key: string]: unknown;
}

interface ClusterStatusResponse extends WazuhResponse {
    data: {
        enabled: 'yes' | 'no';
        affected_items: ClusterNode[];
    };
}

describe('Wazuh Cluster API Flow', () => {
    // Create documentation
    let documentation = '# Wazuh Cluster API Test Results\n\n';
    const appendToDoc = (section: string, content: WazuhResponse | { message: string }): void => {
        documentation += `## ${section}\n\`\`\`json\n${JSON.stringify(content, null, 2)}\n\`\`\`\n\n`;
    };

    let isClusterEnabled: boolean = false;

    test('should get cluster status', async () => {
        const response = await makeAuthorizedRequest('/cluster/status');
        
        expect(response).toBeDefined();
        if (!response) {
            throw new Error('Response is null');
        }

        const typedResponse = response as ClusterStatusResponse;
        expect(typedResponse.data).toBeDefined();
        
        appendToDoc('Cluster Status', typedResponse);
        
        // Store cluster status for subsequent tests
        isClusterEnabled = typedResponse.data.enabled === 'yes';
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
        if (!response) {
            throw new Error('Response is null');
        }

        const typedResponse = response as WazuhResponse;
        expect(typedResponse.data).toBeDefined();
        
        appendToDoc('Cluster Nodes', typedResponse);
    }, 30000);

    test('should get cluster health if enabled', async () => {
        if (!isClusterEnabled) {
            console.log('Cluster is disabled, skipping health test');
            appendToDoc('Cluster Health', { message: 'Test skipped - Cluster is disabled' });
            return;
        }

        const response = await makeAuthorizedRequest('/cluster/healthcheck');
        
        expect(response).toBeDefined();
        if (!response) {
            throw new Error('Response is null');
        }

        const typedResponse = response as WazuhResponse;
        expect(typedResponse.data).toBeDefined();
        
        appendToDoc('Cluster Health', typedResponse);
    }, 30000);

    afterAll(() => {
        // Write documentation to file
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
