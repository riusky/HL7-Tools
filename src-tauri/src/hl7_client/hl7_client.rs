use chrono::Local;
use reqwest::Client;
use reqwest::header::{CONTENT_TYPE, CONTENT_LANGUAGE};
use rusthl7::message::Message;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;
use std::net::SocketAddr;
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Hl7ClientError {
    details: String,
}

impl Hl7ClientError {
    pub fn new(msg: &str) -> Hl7ClientError {
        Hl7ClientError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for Hl7ClientError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for Hl7ClientError {
    fn description(&self) -> &str {
        &self.details
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
// 枚举来定义发送方式
pub enum SendMethod {
    Tcp,
    Http,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Hl7Client {
    message: String,
    server_address: String,
    port: u16,
    method: SendMethod, // 发送方式
}

impl Hl7Client {
    // 实现一个 new 方法来创建 Hl7Client 实例
    pub fn new(message: String, server_address: String, port: u16, method: SendMethod) -> Hl7Client {
        Hl7Client {
            message,
            server_address,
            port,
            method,
        }
    }

    // 发送 HL7 消息通过 TCP
    pub async fn send_hl7_message_tcp(&self) -> Result<String, Box<dyn Error>> {
        let mut status_log = String::new();
        let hl7_message_formatted: String = self
            .message
            .lines()
            .map(|line| line.trim())
            .collect::<Vec<&str>>()
            .join("\r");
        let hl7_message = Message::new(&hl7_message_formatted);
        let hl7_bytes = hl7_message.to_string().into_bytes();

        let addr = format!("{}:{}", self.server_address, self.port);
        let addr: SocketAddr = addr.parse()?;
        let mut stream = TcpStream::connect(&addr).await?;
        status_log.push_str(&format!(
            "[{}] Connected to {}\n",
            Local::now().format("%H:%M:%S"),
            addr
        ));

        stream.write_all(&hl7_bytes).await?;
        status_log.push_str(&format!(
            "[{}] Message sent\n",
            Local::now().format("%H:%M:%S")
        ));

        let mut response = String::new();
        let mut buffer = [0; 1024];
        let mut total_read = 0;
        loop {
            let n = stream.read(&mut buffer).await?;
            if n == 0 {
                break; // No more data
            }
            total_read += n;
            response.push_str(&String::from_utf8_lossy(&buffer[..n]));
        }

        if total_read > 0 {
            status_log.push_str(&format!(
                "[{}] ACK Received:\n{}\n",
                Local::now().format("%H:%M:%S"),
                response
            ));
        } else {
            return Err(Box::new(Hl7ClientError::new("No response from server.")));
        }

        stream.shutdown().await?;
        status_log.push_str(&format!(
            "[{}] Connection to {} has been closed\n",
            Local::now().format("%H:%M:%S"),
            addr
        ));

        Ok(status_log)
    }

    // 发送 HL7 消息通过 HTTP
    pub async fn send_hl7_message_http(&self) -> Result<String, Box<dyn Error>> {
        let mut status_log = String::new();
        let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(10)) // 设置超时时间为10秒
        .build()?;
        let hl7_message_formatted: String = self
            .message
            .lines()
            .map(|line| line.trim())
            .collect::<Vec<&str>>()
            .join("\r");
        let hl7_message = Message::new(&hl7_message_formatted);

        let hl7_bytes: Vec<u8> = hl7_message.to_string().into_bytes();
        let addr = format!("http://{}:{}", self.server_address, self.port);
        status_log.push_str(&format!(
            "[{}] Sending HTTP request to {}\n",
            Local::now().format("%H:%M:%S"),
            &addr
        ));
        let res = client.post(&addr).body(hl7_bytes).send().await;
        match res {
            Ok(response) => {
                if response.status().is_success() {
                    let resp_text = response.text().await?;
                    status_log.push_str(&format!(
                        "[{}] HTTP Response Received:\n{}\n",
                        Local::now().format("%H:%M:%S"),
                        resp_text
                    ));
                    Ok(status_log)
                } else {
                    status_log.push_str(&format!(
                        "[{}] Failed with status: {}\n",
                        Local::now().format("%H:%M:%S"),
                        response.status()
                    ));
                    Err(Box::new(Hl7ClientError::new(&status_log)))
                }
            }
            Err(e) => {
                let error_message = if e.is_timeout() {
                    "Request timed out".to_string()
                } else {
                    format!("HTTP request failed: {}", e)
                };
                status_log.push_str(&format!(
                    "[{}] {}\n",
                    Local::now().format("%H:%M:%S"),
                    error_message
                ));
                Err(Box::new(Hl7ClientError::new(&status_log)))
            }
        }

        // Ok(res_text)
    }

    // 发送消息根据配置的方法
    pub async fn send_message(&self) -> Result<String, Box<dyn Error>> {
        match self.method {
            SendMethod::Tcp => self.send_hl7_message_tcp().await,
            SendMethod::Http => self.send_hl7_message_http().await,
        }
    }
}
