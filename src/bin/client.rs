use std::env;
use std::process;
use std::fs;
use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use native_tls::TlsConnector;
use tokio_native_tls::TlsConnector as TokioTlsConnector;
use std::time::{SystemTime, UNIX_EPOCH, Duration};
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use serde_json;
use sha2::{Sha256, Digest};
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use std::path::{Path, PathBuf};
use std::io::Write;
use tokio::time::sleep;

const MAX_RETRIES: u32 = 3;
const RETRY_DELAY: Duration = Duration::from_secs(1);
const RECONNECT_DELAY: Duration = Duration::from_secs(2);
const SESSION_FILE: &str = "session.json";
const WQL_QUERIES_DIR: &str = "wql_queries";
const BUFFER_SIZE: usize = 8192;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Response {
    status: bool,
    data: String,
    session_id: String,
    timestamp: u64,
    signature: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct AuthRequest {
    client_id: String,
    timestamp: u64,
    nonce: String,
    signature: String,
    session_id: Option<String>,
    wql_query: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct SessionInfo {
    session_id: String,
    client_id: String,
    created_at: u64,
    last_used: u64,
}

struct Client {
    client_id: String,
    client_key: String,
    server_key: String,
    session: Option<SessionInfo>,
}

impl Client {
    fn new(client_id: String, client_key: String, server_key: String) -> Self {
        let session = Self::load_session(&client_id);
        Self {
            client_id,
            client_key,
            server_key,
            session,
        }
    }

    fn load_session(client_id: &str) -> Option<SessionInfo> {
        if let Ok(content) = fs::read_to_string(SESSION_FILE) {
            if let Ok(session) = serde_json::from_str::<SessionInfo>(&content) {
                let now = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs();
                
                if now - session.created_at <= 3600 && session.client_id == client_id {
                    println!("Loaded existing session: {}", session.session_id);
                    return Some(session);
                }
            }
        }
        None
    }

    fn save_session(&self) -> Result<()> {
        if let Some(session) = &self.session {
            let content = serde_json::to_string_pretty(session)?;
            fs::write(SESSION_FILE, content)?;
            println!("Session saved: {}", session.session_id);
        }
        Ok(())
    }

    fn sign_request(&self, data: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        hasher.update(self.client_key.as_bytes());
        BASE64.encode(hasher.finalize())
    }

    fn verify_response(&self, response_data: &str, signature: &str) -> bool {
        let mut hasher = Sha256::new();
        hasher.update(response_data.as_bytes());
        hasher.update(self.server_key.as_bytes());
        let expected = BASE64.encode(hasher.finalize());
        expected == signature
    }

    async fn stream_response(
        stream: &mut tokio_native_tls::TlsStream<TcpStream>,
    ) -> Result<String> {
        let mut response_data = Vec::new();
        let mut buffer = vec![0u8; BUFFER_SIZE];
        let mut total_bytes = 0;
        
        print!("\rReceiving data: 0 bytes");
        std::io::stdout().flush()?;

        loop {
            match stream.read(&mut buffer).await {
                Ok(0) => {
                    if total_bytes == 0 {
                        return Err("Connection closed by server".into());
                    }
                    break;
                },
                Ok(n) => {
                    response_data.extend_from_slice(&buffer[..n]);
                    total_bytes += n;
                    print!("\rReceiving data: {} bytes", total_bytes);
                    std::io::stdout().flush()?;
                }
                Err(e) => return Err(format!("Failed to read response: {}", e).into()),
            }
        }
        println!("\nReceived total: {} bytes", total_bytes);

        String::from_utf8(response_data)
            .map_err(|e| format!("Invalid UTF-8 sequence: {}", e).into())
    }

    async fn send_request(
        &mut self, 
        stream: &mut tokio_native_tls::TlsStream<TcpStream>,
        wql_query: String
    ) -> Result<Response> {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)?
            .as_secs();
        
        let nonce = Uuid::new_v4().to_string();
        
        let data_to_sign = format!("{}:{}:{}", 
            self.client_id,
            timestamp,
            nonce
        );

        let signature = self.sign_request(&data_to_sign);

        let request = AuthRequest {
            client_id: self.client_id.clone(),
            timestamp,
            nonce,
            signature,
            session_id: self.session.as_ref().map(|s| s.session_id.clone()),
            wql_query,
        };

        let request_json = serde_json::to_string(&request)?;
        println!("Sending request...");
        stream.write_all(request_json.as_bytes()).await?;
        stream.flush().await?;

        println!("Waiting for response...");
        let response_str = Self::stream_response(stream).await?;
        
        let mut response: Response = serde_json::from_str(&response_str)?;
        
        let signature = response.signature.clone();
        response.signature = String::new();
        let response_data = serde_json::to_string(&response)?;
        
        if !self.verify_response(&response_data, &signature) {
            return Err("Invalid response signature".into());
        }

        response.signature = signature;

        self.session = Some(SessionInfo {
            session_id: response.session_id.clone(),
            client_id: self.client_id.clone(),
            created_at: timestamp,
            last_used: timestamp,
        });
        self.save_session()?;

        Ok(response)
    }
}

fn get_wql_query_files() -> Result<Vec<PathBuf>> {
    let mut query_files = Vec::new();
    for entry in fs::read_dir(WQL_QUERIES_DIR)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.extension().map_or(false, |ext| ext == "json") {
            query_files.push(path);
        }
    }
    Ok(query_files)
}

async fn connect_with_retry(
    addr: &str,
    connector: &TokioTlsConnector,
) -> Result<tokio_native_tls::TlsStream<TcpStream>> {
    let mut last_error = None;
    for _ in 0..MAX_RETRIES {
        match TcpStream::connect(addr).await {
            Ok(stream) => {
                return Ok(connector.connect("localhost", stream).await?);
            }
            Err(e) => {
                last_error = Some(e);
                sleep(RETRY_DELAY).await;
            }
        }
    }
    Err(format!("Failed to connect after {} retries: {:?}", MAX_RETRIES, last_error.unwrap()).into())
}

#[tokio::main]
async fn main() -> Result<()> {
    let server_addr = env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Usage: client <server_address:port>");
        eprintln!("Example: client 192.168.1.100:8080");
        process::exit(1);
    });

    println!("Loading WQL query files...");
    let query_files = get_wql_query_files()?;
    if query_files.is_empty() {
        eprintln!("No WQL query files found in {} directory", WQL_QUERIES_DIR);
        process::exit(1);
    }

    let mut client = Client::new(
        "client1".to_string(),
        "test_key_1".to_string(),
        "server_key".to_string(),
    );
    
    let connector = TlsConnector::builder()
        .danger_accept_invalid_certs(true)
        .build()?;
    let connector = TokioTlsConnector::from(connector);
    
    let output_dir = "query_results";
    fs::create_dir_all(output_dir)?;

    for query_file in query_files {
        println!("\n執行查詢: {:?}", query_file);
        
        let query_content = fs::read_to_string(&query_file)?;
        
        // 為每個查詢建立新的連接
        println!("Connecting to server at {}...", server_addr);
        let mut stream = connect_with_retry(&server_addr, &connector).await?;
        println!("TLS connection established");
        
        let response = client.send_request(&mut stream, query_content).await?;
        
        if response.status {
            let query_name = query_file.file_stem().unwrap().to_string_lossy();
            let output_file = format!("{}/{}_{}.json", 
                output_dir,
                query_name,
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)?
                    .as_secs()
            );
            
            fs::write(&output_file, &response.data)?;
            println!("查詢結果已保存到: {}", output_file);
        } else {
            eprintln!("查詢失敗: {}", response.data);
        }
        
        // 等待一段時間再進行下一個查詢
        sleep(RECONNECT_DELAY).await;
    }

    println!("\n所有查詢已完成");
    Ok(())
}
