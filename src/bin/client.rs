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
use std::path::Path;

const MAX_RETRIES: u32 = 3;
const RETRY_DELAY: Duration = Duration::from_secs(1);
const SESSION_FILE: &str = "session.json";

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

    async fn send_request(&mut self, stream: &mut tokio_native_tls::TlsStream<TcpStream>) -> Result<Response> {
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
        };

        let request_json = serde_json::to_string(&request)?;
        println!("Sending request: {}", request_json);
        stream.write_all(request_json.as_bytes()).await?;
        
        let mut response = Vec::new();
        stream.read_to_end(&mut response).await?;
        
        if response.is_empty() {
            return Err("Empty response from server".into());
        }

        let response_str = String::from_utf8(response)?;
        println!("Received response: {}", response_str);
        
        let mut response: Response = serde_json::from_str(&response_str)?;
        
        // 驗證響應
        let signature = response.signature.clone();
        response.signature = String::new();
        let response_data = serde_json::to_string(&response)?;
        
        if !self.verify_response(&response_data, &signature) {
            return Err("Invalid response signature".into());
        }

        response.signature = signature;

        // 更新會話信息
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
                tokio::time::sleep(RETRY_DELAY).await;
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

    println!("Connecting to server at {}...", server_addr);
    
    let mut client = Client::new(
        "client1".to_string(),
        "test_key_1".to_string(),
        "server_key".to_string(),
    );
    
    let connector = TlsConnector::builder()
        .danger_accept_invalid_certs(true)
        .build()?;
    let connector = TokioTlsConnector::from(connector);
    
    let mut stream = connect_with_retry(&server_addr, &connector).await?;
    println!("TLS connection established");

    let response = client.send_request(&mut stream).await?;
    
    if response.status {
        println!("\n成功獲取數據:");
        println!("{}", response.data);
    } else {
        eprintln!("\n錯誤:");
        eprintln!("{}", response.data);
        process::exit(1);
    }

    Ok(())
}
