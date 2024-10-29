import { makeAuthorizedRequest } from '../utils/auth-helper';
import { TestDocumenter } from './utils/test-documenter';

describe('Wazuh Overview API Through Rust Proxy', () => {
    let documenter: TestDocumenter;

    beforeAll(() => {
        TestDocumenter.setTimestamp();  // 設置全域時間戳記
        TestDocumenter.resetInstance();
        documenter = TestDocumenter.getInstance('Wazuh Overview API');
    });

    test('should proxy get overview request', async () => {
        const testCase = {
            name: 'Get Overview',
            endpoint: '/manager/info'
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
