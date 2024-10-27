import { makeAuthorizedRequest } from '../utils/auth-helper';

describe('Wazuh Logtest API Flow', () => {
    // Create documentation
    let documentation = '# Wazuh Logtest API Test Results\n\n';
    const appendToDoc = (section: string, content: any) => {
        documentation += `## ${section}\n\`\`\`json\n${JSON.stringify(content, null, 2)}\n\`\`\`\n\n`;
    };

    test('should start logtest session', async () => {
        const testLog = {
            event: "Jun 24 11:54:23 hostname sshd[12345]: Accepted password for user from 192.168.1.1 port 54321",
            log_format: "syslog",
            location: "/var/log/syslog"
        };

        const response = await makeAuthorizedRequest(
            '/logtest',
            'PUT',
            testLog
        );
        
        expect(response).toBeDefined();
        expect(response.data).toBeDefined();
        
        appendToDoc('Logtest Session Start', {
            description: 'Starting a new logtest session',
            request: testLog,
            response: response
        });
    }, 30000);

    test('should test log with custom rules', async () => {
        const testConfig = {
            event: "Jun 24 11:54:23 hostname sshd[12345]: Failed password for invalid user test from 192.168.1.1 port 54321",
            log_format: "syslog",
            location: "/var/log/auth.log"
        };

        const response = await makeAuthorizedRequest(
            '/logtest/run',
            'PUT',
            testConfig
        );
        
        expect(response).toBeDefined();
        expect(response.data).toBeDefined();
        
        appendToDoc('Logtest With Custom Rules', {
            description: 'Testing log analysis with custom rules',
            request: testConfig,
            response: response
        });
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
            path.join(docsDir, 'wazuh-logtest-responses.md'),
            documentation
        );
    });
});
