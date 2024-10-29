import { makeAuthorizedRequest } from '../utils/auth-helper';
import {TestDocumenter} from "@/app/utils/test-documenter";

jest.setTimeout(30000); // 增加超時時間到30秒

describe('Wazuh CIS-CAT API Through Rust Proxy', () => {
    let documenter: TestDocumenter;

    beforeAll(() => {
        TestDocumenter.setTimestamp();
        TestDocumenter.resetInstance();
        documenter = TestDocumenter.getInstance('Wazuh CIS-CAT API');
    });

    test.skip('should proxy get ciscat results request', async () => {
        const testCase = {
            name: 'Get CIS-CAT Results',
            endpoint: '/ciscat/000/results'
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
