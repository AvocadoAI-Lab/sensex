import { makeAuthorizedRequest } from '../utils/auth-helper';
import type { AgentsResponse } from '../types/agents';
import { isAgentsResponse } from '../types/agents';
import { TestDocumenter } from './utils/test-documenter';

describe('Wazuh Agents API Through Rust Proxy', () => {
    let firstAgentId: string | undefined;
    let documenter: TestDocumenter;

    beforeAll(() => {
        TestDocumenter.setTimestamp();  // 設置全域時間戳記
        TestDocumenter.resetInstance();
        documenter = TestDocumenter.getInstance('Wazuh Agents API');
    });

    test('should proxy get all agents request', async () => {
        const testCase = {
            name: 'Get All Agents',
            endpoint: '/agents'
        };

        documenter.startTestCase(testCase);

        const response = await makeAuthorizedRequest<AgentsResponse>(testCase.endpoint);
        
        expect(response).toBeDefined();
        
        if (!response || !isAgentsResponse(response)) {
            const error = 'Invalid response format';
            documenter.logError(testCase, error);
            throw new Error(error);
        }

        documenter.logResponse(testCase, response);

        if (response.data.affected_items.length > 0) {
            const firstAgent = response.data.affected_items[0];
            if (firstAgent.id) {
                firstAgentId = firstAgent.id;
                console.log('Found first agent ID:', firstAgentId);
            }
        }
    });

    test('should proxy get specific agent details', async () => {
        if (!firstAgentId) {
            const testCase = {
                name: 'Get Specific Agent',
                endpoint: '/agents'
            };
            const error = 'No agents available to test';
            console.log(error);
            documenter.logError(testCase, error);
            return;
        }

        const testCase = {
            name: 'Get Specific Agent',
            endpoint: `/agents?agents_list=${firstAgentId}`,
            requestDetails: { agentId: firstAgentId }
        };

        documenter.startTestCase(testCase);

        const response = await makeAuthorizedRequest<AgentsResponse>(testCase.endpoint);
        
        expect(response).toBeDefined();
        if (!response || !isAgentsResponse(response)) {
            const error = 'Invalid response format';
            documenter.logError(testCase, error);
            throw new Error(error);
        }

        documenter.logResponse(testCase, response);
    });

    afterAll(() => {
        documenter.save();
    });
});
