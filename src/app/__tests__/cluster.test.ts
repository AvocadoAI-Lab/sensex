import { makeAuthorizedRequest } from '../utils/auth-helper';
import { TestDocumenter } from '../utils/test-documenter';

jest.setTimeout(30000); // 增加超時時間到30秒

describe('Wazuh Cluster API Through Rust Proxy', () => {
    let documenter: TestDocumenter;

    beforeAll(() => {
        documenter = new TestDocumenter('Wazuh Cluster API');
    });

    test('should proxy get cluster status request', async () => {
        const testCase = {
            name: 'Get Cluster Status',
            endpoint: '/cluster/status'
        };

        documenter.startTestCase(testCase);

        try {
            const response = await makeAuthorizedRequest(testCase.endpoint);
            
            expect(response).toBeDefined();
            if (!response) {
                const error = 'Invalid response format';
                documenter.logError(testCase, error);
                throw new Error(error);
            }

            documenter.logResponse(testCase, response);
        } catch (error) {
            if (error instanceof Error) {
                const statusCodeMatch = error.message.match(/Request failed: (\d+)/);
                const statusCode = statusCodeMatch ? parseInt(statusCodeMatch[1]) : undefined;
                
                documenter.logError(
                    testCase,
                    error.message,
                    statusCode,
                    error
                );
            }
            throw error;
        }
    });

    test('should proxy get cluster local info request', async () => {
        const testCase = {
            name: 'Get Cluster Local Info',
            endpoint: '/cluster/local/info'
        };

        documenter.startTestCase(testCase);

        try {
            const response = await makeAuthorizedRequest(testCase.endpoint);
            
            expect(response).toBeDefined();
            if (!response) {
                const error = 'Invalid response format';
                documenter.logError(testCase, error);
                throw new Error(error);
            }

            documenter.logResponse(testCase, response);
        } catch (error) {
            if (error instanceof Error) {
                const statusCodeMatch = error.message.match(/Request failed: (\d+)/);
                const statusCode = statusCodeMatch ? parseInt(statusCodeMatch[1]) : undefined;
                
                documenter.logError(
                    testCase,
                    error.message,
                    statusCode,
                    error
                );
            }
            throw error;
        }
    });

    afterAll(() => {
        documenter.save();
    });
});
