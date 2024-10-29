import { makeAuthorizedRequest } from '../utils/auth-helper';
import {TestDocumenter} from "@/app/utils/test-documenter";

jest.setTimeout(30000); // 增加超時時間到30秒

describe('Wazuh Statistics API Through Rust Proxy', () => {
    let documenter: TestDocumenter;

    beforeAll(() => {
        TestDocumenter.setTimestamp();
        TestDocumenter.resetInstance();
        documenter = TestDocumenter.getInstance('Wazuh Statistics API');
    });

    test.skip('should proxy get statistics request', async () => {
        const testCase = {
            name: 'Get Statistics',
            endpoint: '/manager/stats'
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

    test.skip('should proxy get statistics hourly request', async () => {
        const testCase = {
            name: 'Get Hourly Statistics',
            endpoint: '/manager/stats/hourly'
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

    test.skip('should proxy get statistics weekly request', async () => {
        const testCase = {
            name: 'Get Weekly Statistics',
            endpoint: '/manager/stats/weekly'
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
