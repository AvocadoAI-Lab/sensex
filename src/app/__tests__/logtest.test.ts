import { makeAuthorizedRequest } from '../utils/auth-helper';

describe('Wazuh Logtest API Flow', () => {
    // Create documentation
    let documentation = '# Wazuh Logtest API Test Results\n\n';
    const appendToDoc = (section: string, content: any) => {
        documentation += `## ${section}\n\`\`\`json\n${JSON.stringify(content, null, 2)}\n\`\`\`\n\n`;
    };

    let sessionToken: string;

    beforeAll(async () => {
        // Start a new logtest session
        const startSession = await makeAuthorizedRequest('/logtest', 'PUT', {
            event: "Jun 24 11:54:23 linux-agent sshd[29205]: Invalid user foo from 183.3.202.111",
            location: "/var/log/auth.log",
            log_format: "syslog"
        });

        if (startSession.data && startSession.data.token) {
            sessionToken = startSession.data.token;
            console.log('Logtest session started');
            appendToDoc('Session Start', startSession);
        } else {
            console.log('Failed to start logtest session');
        }
    }, 30000);

    test('should test log analysis', async () => {
        if (!sessionToken) {
            console.log('No session token available, skipping log analysis test');
            appendToDoc('Log Analysis', { message: 'Test skipped - No session token available' });
            return;
        }

        const testLog = {
            event: "Jun 24 11:54:23 linux-agent sshd[29205]: Failed password for invalid user foo from 183.3.202.111 port 48928 ssh2",
            location: "/var/log/auth.log",
            log_format: "syslog",
            token: sessionToken
        };

        const response = await makeAuthorizedRequest('/logtest', 'PUT', testLog);
        
        expect(response).toBeDefined();
        expect(response.data).toBeDefined();
        
        appendToDoc('Log Analysis', {
            request: testLog,
            response: response
        });
    }, 30000);

    test('should end logtest session', async () => {
        if (!sessionToken) {
            console.log('No session token available, skipping session end test');
            appendToDoc('Session End', { message: 'Test skipped - No session token available' });
            return;
        }

        const response = await makeAuthorizedRequest('/logtest/remove', 'DELETE', {
            token: sessionToken,
            location: "/var/log/auth.log"  // Required for session cleanup
        });
        
        expect(response).toBeDefined();
        expect(response.data).toBeDefined();
        
        appendToDoc('Session End', response);
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
