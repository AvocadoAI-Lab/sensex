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
    private timestamp: string;

    constructor(testName: string) {
        this.documentation = `# ${testName} Test Results\n\n`;
        this.yamlStructure = '';
        this.logs = [];
        this.testName = testName;
        
        // 設置時間戳記
        const now = new Date();
        const year = now.getFullYear();
        const month = String(now.getMonth() + 1).padStart(2, '0');
        const day = String(now.getDate()).padStart(2, '0');
        const hour = String(now.getHours()).padStart(2, '0');
        const minute = String(now.getMinutes()).padStart(2, '0');
        this.timestamp = `${year}-${month}-${day}T${hour}-${minute}`;
        
        // 使用絕對路徑
        const projectRoot = path.resolve(__dirname, '..', '..');
        this.docsDir = path.join(projectRoot, 'docs');
        
        // 確保docs目錄存在
        if (!fs.existsSync(this.docsDir)) {
            fs.mkdirSync(this.docsDir, { recursive: true });
        }
        
        // 確保時間戳記目錄存在
        const timestampDir = path.join(this.docsDir, this.timestamp);
        if (!fs.existsSync(timestampDir)) {
            fs.mkdirSync(timestampDir, { recursive: true });
        }
        
        const baseFilename = this.testName.toLowerCase().replace(/\s+/g, '-');
        this.testDir = path.join(timestampDir, baseFilename);
        
        // 確保測試目錄存在
        if (!fs.existsSync(this.testDir)) {
            fs.mkdirSync(this.testDir, { recursive: true });
        }

        console.log('Docs directory:', this.docsDir);
        console.log('Test directory:', this.testDir);
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
        try {
            // 確保目錄存在
            if (!fs.existsSync(this.testDir)) {
                fs.mkdirSync(this.testDir, { recursive: true });
            }

            // 寫入檔案
            const responsesPath = path.join(this.testDir, 'responses.md');
            fs.writeFileSync(responsesPath, this.documentation);
            console.log('Saved responses to:', responsesPath);

            const structurePath = path.join(this.testDir, 'structure.yaml');
            fs.writeFileSync(structurePath, this.yamlStructure);
            console.log('Saved structure to:', structurePath);

            const logContent = this.logs
                .map(log => JSON.stringify(log, null, 2))
                .join('\n\n');
            const logPath = path.join(this.testDir, 'test-log.json');
            fs.writeFileSync(logPath, `[\n${logContent}\n]`);
            console.log('Saved logs to:', logPath);

            console.log('All documentation saved successfully');
        } catch (error) {
            console.error('Error saving documentation:', error);
            console.error('Test directory:', this.testDir);
            console.error('Current working directory:', process.cwd());
            throw error;  // 重新拋出錯誤以便測試可以捕捉到
        }
    }
}
