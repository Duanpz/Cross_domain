use tonic::transport::Channel;
use std::collections::HashMap;
use std::sync::Arc;

// 重新导入生成的 protobuf 类型
mod pb {
    include!(concat!(env!("OUT_DIR"), "/chuangshi.rs"));
}

use pb::{MetadataServiceClient, CreateRequest, CreateResponse, ReadRequest, ReadResponse, DeleteRequest, DeleteResponse, ListRequest, ListResponse, GetAttributesRequest, GetAttributesResponse, UpdateMetadataRequest, UpdateMetadataResponse};

/// 客户端配置
#[derive(Debug, Clone)]
pub struct ClientConfig {
    pub endpoint: String,
    pub timeout: std::time::Duration,
    pub retry_attempts: u32,
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            endpoint: "http://127.0.0.1:50051".to_string(),
            timeout: std::time::Duration::from_secs(30),
            retry_attempts: 3,
        }
    }
}

/// 创世 NDFS 客户端
pub struct ChuanShiClient {
    client: MetadataServiceClient<Channel>,
    config: ClientConfig,
}

impl ChuanShiClient {
    /// 创建新的客户端实例
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Self::with_config(ClientConfig::default()).await
    }

    /// 使用配置创建客户端
    pub async fn with_config(config: ClientConfig) -> Result<Self, Box<dyn std::error::Error>> {
        let channel = Channel::from_shared(config.endpoint.clone())?
            .timeout(config.timeout)
            .connect()
            .await?;
        let client = MetadataServiceClient::new(channel);
        Ok(ChuanShiClient { client, config })
    }

    /// 重试机制
    async fn retry_operation<T, F>(&self, operation: F) -> Result<T, Box<dyn std::error::Error>>
    where
        F: FnOnce() -> tonic::Request<T>,
    {
        let mut last_error = None;
        
        for attempt in 0..self.config.retry_attempts {
            match operation().await {
                Ok(response) => return Ok(response),
                Err(e) => {
                    last_error = Some(e);
                    if attempt < self.config.retry_attempts - 1 {
                        tokio::time::sleep(tokio::time::Duration::from_millis(100 * (attempt + 1))).await;
                    }
                }
            }
        }
        
        Err(last_error.unwrap_or_else(|| "Operation failed after all retries".into()))
    }

    /// 创建文件
    pub async fn create_file(&mut self, path: &str, data: Vec<u8>) -> Result<bool, Box<dyn std::error::Error>> {
        let req = CreateRequest {
            path: path.to_string(),
            data,
            metadata: HashMap::new(),
            node_id: "".to_string(),
            replicas: vec![],
            replication_factor: 0,
        };
        
        let resp = self.client.create_file(req).await?.into_inner();
        Ok(resp.success)
    }

    /// 读取文件
    pub async fn read_file(&mut self, path: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let req = ReadRequest {
            path: path.to_string(),
            offset: 0,
            length: 0,
        };
        let resp = self.client.read_file(req).await?.into_inner();
        Ok(resp.data)
    }

    /// 删除文件
    pub async fn delete_file(&mut self, path: &str, force: bool) -> Result<bool, Box<dyn std::error::Error>> {
        let req = DeleteRequest {
            path: path.to_string(),
            force,
        };
        let resp = self.client.delete_file(req).await?.into_inner();
        Ok(resp.success)
    }

    /// 列出目录
    pub async fn list_directory(&mut self, path: &str, recursive: bool) -> Result<ListResponse, Box<dyn std::error::Error>> {
        let req = ListRequest {
            path: path.to_string(),
            recursive,
        };
        let resp = self.client.list_directory(req).await?.into_inner();
        Ok(resp)
    }

    /// 获取文件属性
    pub async fn get_file_attributes(&mut self, path: &str) -> Result<GetAttributesResponse, Box<dyn std::error::Error>> {
        let req = GetAttributesRequest {
            path: path.to_string(),
        };
        let resp = self.client.get_file_attributes(req).await?.into_inner();
        Ok(resp)
    }

    /// 更新文件元数据
    pub async fn update_file_metadata(&mut self, path: &str, metadata: HashMap<String, String>) -> Result<bool, Box<dyn std::error::Error>> {
        let req = UpdateMetadataRequest {
            path: path.to_string(),
            metadata,
            node_id: "".to_string(),
        };
        let resp = self.client.update_file_metadata(req).await?.into_inner();
        Ok(resp.success)
    }

    /// 检查文件是否存在
    pub async fn file_exists(&mut self, path: &str) -> Result<bool, Box<dyn std::error::Error>> {
        let req = GetAttributesRequest {
            path: path.to_string(),
        };
        let resp = self.client.get_file_attributes(req).await?.into_inner();
        Ok(resp.exists)
    }

    /// 获取文件大小
    pub async fn get_file_size(&mut self, path: &str) -> Result<i64, Box<dyn std::error::Error>> {
        let req = GetAttributesRequest {
            path: path.to_string(),
        };
        let resp = self.client.get_file_attributes(req).await?.into_inner();
        if let Some(file_info) = resp.file_info {
            Ok(file_info.size)
        } else {
            Err("File not found".into())
        }
    }

    /// 批量创建文件
    pub async fn batch_create_files(&mut self, files: Vec<(&str, Vec<u8>)>) -> Result<Vec<bool>, Box<dyn std::error::Error>> {
        let mut results = Vec::new();
        
        for (path, data) in files {
            let success = self.create_file(path, data).await?;
            results.push(success);
        }
        
        Ok(results)
    }

    /// 批量读取文件
    pub async fn batch_read_files(&mut self, paths: Vec<&str>) -> Result<Vec<Vec<u8>>, Box<dyn std::error::Error>> {
        let mut results = Vec::new();
        
        for path in paths {
            let data = self.read_file(path).await?;
            results.push(data);
        }
        
        Ok(results)
    }
}