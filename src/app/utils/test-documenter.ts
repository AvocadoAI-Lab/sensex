import fs from 'fs';
import path from 'path';

export interface TestLog {
    timestamp: string;
    testName: string;
    endpoint: string;
    requestDetails?: unknown;
    response?: unknown;
    error?: string;
    statusCode?: number;
    errorDetails?: unknown;
}

export interface TestCase {
    name: string;
    endpoint: string;
    requestDetails?: unknown;
}

export class TestDocumenter {
    private documentation: string;
    private yamlStructure: string;
    private logs: TestLog[];
    private testName: string;
    private docsDir: string;
    private testDir: string;
    private static instance: TestDocumenter | null = null;
    private static timestamp: string | null = null;

    private constructor(testName: string) {
        this.documentation = `# ${testName} Test Results\n\n`;
        this.yamlStructure = '';
        this.logs = [];
        this.testName = testName;
        this.docsDir = path.join(__dirname, '..', 'docs');
        
        const baseFilename = this.testName.toLowerCase().replace(/\s+/g, '-');
        this.testDir = path.join(this.docsDir, TestDocumenter.timestamp!, baseFilename);
    }

    public static setTimestamp(): void {
        // 只取到分鐘級別的時間戳記
        const now = new Date();
        const year = now.getFullYear();
        const month = String(now.getMonth() + 1).padStart(2, '0');
        const day = String(now.getDate()).padStart(2, '0');
        const hour = String(now.getHours()).padStart(2, '0');
        const minute = String(now.getMinutes()).padStart(2, '0');
        
        TestDocumenter.timestamp = `${year}-${month}-${day}T${hour}-${minute}`;
    }

    public static getInstance(testName: string): TestDocumenter {
        if (!TestDocumenter.timestamp) {
            throw new Error('Timestamp not set. Call TestDocumenter.setTimestamp() before creating instances.');
        }
        if (!TestDocumenter.instance) {
            TestDocumenter.instance = new TestDocumenter(testName);
        }
        return TestDocumenter.instance;
    }

    public static resetInstance(): void {
        TestDocumenter.instance = null;
    }

    private analyzeStructure(obj: unknown, prefix: string = ''): string {
        let structure = '';
        
        if (Array.isArray(obj)) {
            if (obj.length === 0) {
                return `${prefix}type: array\n${prefix}items: []\n`;
            }
            structure += `${prefix}type: array\n${prefix}items:\n`;
            if (typeof obj[0] === 'object' && obj[0] !== null) {
                structure += this.analyzeStructure(obj[0], `${prefix}  `);
            } else {
                structure += `${prefix}  type: ${typeof obj[0]}\n`;
            }
        } else if (typeof obj === 'object' && obj !== null) {
            structure += `${prefix}type: object\n${prefix}properties:\n`;
            Object.entries(obj as Record<string, unknown>).forEach(([key, value]) => {
                structure += `${prefix}  ${key}:\n`;
                if (value === null) {
                    structure += `${prefix}    type: null\n`;
                } else if (typeof value === 'object') {
                    structure += this.analyzeStructure(value, `${prefix}    `);
                } else {
                    structure += `${prefix}    type: ${typeof value}\n`;
                    if (typeof value === 'string' && value) {
                        structure += `${prefix}    example: "${value}"\n`;
                    } else if (typeof value === 'number') {
                        structure += `${prefix}    example: ${value}\n`;
                    }
                }
            });
        } else if (typeof obj === 'string') {
            structure += `${prefix}type: string\n`;
            if (obj) {
                structure += `${prefix}example: "${obj}"\n`;
            }
        } else if (typeof obj === 'number') {
            structure += `${prefix}type: number\n`;
            structure += `${prefix}example: ${obj}\n`;
        } else if (typeof obj === 'boolean') {
            structure += `${prefix}type: boolean\n`;
            structure += `${prefix}example: ${obj}\n`;
        } else {
            structure += `${prefix}type: ${typeof obj}\n`;
        }
        
        return structure;
    }

    public startTestCase(testCase: TestCase): void {
        this.addTestLog({
            testName: testCase.name,
            endpoint: testCase.endpoint,
            requestDetails: testCase.requestDetails
        });
    }

    public logResponse(testCase: TestCase, response: unknown): void {
        this.addTestLog({
            testName: testCase.name,
            endpoint: testCase.endpoint,
            response
        });
        this.appendToDoc(`${testCase.name} Response`, response);
    }

    public logError(testCase: TestCase, error: string, statusCode?: number, errorDetails?: unknown): void {
        const errorLog = {
            testName: testCase.name,
            endpoint: testCase.endpoint,
            error,
            statusCode,
            errorDetails
        };
        this.addTestLog(errorLog);
        this.appendToDoc(`${testCase.name} Error`, errorLog);
    }

    private addTestLog(log: Omit<TestLog, 'timestamp'>) {
        this.logs.push({
            ...log,
            timestamp: new Date().toISOString()
        });
    }

    private appendToDoc(section: string, content: unknown): void {
        this.documentation += `## ${section}\n\`\`\`json\n${JSON.stringify(content, null, 2)}\n\`\`\`\n\n`;
        this.yamlStructure += `# ${section}\n`;
        this.yamlStructure += this.analyzeStructure(content);
        this.yamlStructure += '---\n\n';
    }

    public save(): void {
        if (!fs.existsSync(this.testDir)) {
            fs.mkdirSync(this.testDir, { recursive: true });
        }

        fs.writeFileSync(
            path.join(this.testDir, 'responses.md'),
            this.documentation
        );

        fs.writeFileSync(
            path.join(this.testDir, 'structure.yaml'),
            this.yamlStructure
        );

        const logContent = this.logs
            .map(log => JSON.stringify(log, null, 2))
            .join('\n\n');
        fs.writeFileSync(
            path.join(this.testDir, 'test-log.json'),
            `[\n${logContent}\n]`
        );

        console.log(`Test documentation saved to: ${this.testDir}`);
    }
}
