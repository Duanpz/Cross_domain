use tokio::net::TcpListener;
use tonic::{transport::Server, Request, Response, Status};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use std::time::SystemTime;
use std::path::Path;

pub mod proto {
    tonic::include_proto!("chuangshi"); // 与 proto 文件中的 package 名一致
}


use proto::{metadata_service_server::MetadataServiceServer, CreateRequest, ReadRequest, DeleteRequest, ListRequest,CreateResponse,ReadResponse,DeleteResponse,ListResponse};

pub struct RegionalMetadataNode {
    metadata: Arc<Mutex<HashMap<String, MetadataEntry>>>,
    data_dir: String,
    region_id: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MetadataEntry {
    pub path: String,
    pub size: u64,
    pub owner: String,
    pub created_at: u64,
    pub updated_at: u64,
    pub location: String, // 存储位置
}

impl RegionalMetadataNode {
    pub fn new(data_dir: String, region_id: String) -> Self {
        Self {
            metadata: Arc::new(Mutex::new(HashMap::new())),
            data_dir,
            region_id,
        }
    }

    async fn create_file_internal(&self, path: &str, size: u64, owner: &str) -> Result<(), Box<dyn std::error::Error>> {
        log::info!("Creating regional file: {} in region: {}", path, self.region_id);
        
        let entry = MetadataEntry {
            path: path.to_string(),
            size,
            owner: owner.to_string(),
            created_at: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            updated_at: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            location: format!("region_{}", self.region_id),
        };

        let mut metadata = self.metadata.lock().await;
        metadata.insert(path.to_string(), entry);
        
        log::info!("Successfully created regional file: {} in region: {}", path, self.region_id);
        Ok(())
    }

    async fn read_file_internal(&self, path: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        log::info!("Reading regional file: {} in region: {}", path, self.region_id);
        
        // 在实际实现中，这里应该从本地存储读取数据
        let data = vec![];
        log::info!("Successfully read regional file: {} in region: {}", path, self.region_id);
        Ok(data)
    }

    async fn delete_file_internal(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        log::info!("Deleting regional file: {} in region: {}", path, self.region_id);
        
        let mut metadata = self.metadata.lock().await;
        if metadata.remove(path).is_some() {
            log::info!("Successfully deleted regional file: {} in region: {}", path, self.region_id);
            Ok(())
        } else {
            log::warn!("Regional file not found for deletion: {} in region: {}", path, self.region_id);
            Err("File not found".into())
        }
    }
}

#[tonic::async_trait]
impl proto::metadata_service_server::MetadataService for RegionalMetadataNode {
    async fn create_file(
        &self,
        request: Request<CreateRequest>,
    ) -> Result<Response<CreateResponse>, Status> {
        let req = request.into_inner();
        
        match self.create_file_internal(&req.path, req.size, &req.owner).await {
            Ok(_) => {
                log::info!("Create regional file success: {}", req.path);
                Ok(Response::new(CreateResponse {
                    success: true,
                    error: String::new(),
                }))
            }
            Err(e) => {
                log::error!("Create regional file failed: {} - {}", req.path, e);
                Ok(Response::new(CreateResponse {
                    success: false,
                    error: e.to_string(),
                }))
            }
        }
    }

    async fn read_file(
        &self,
        request: Request<ReadRequest>,
    ) -> Result<Response<ReadResponse>, Status> {
        let req = request.into_inner();
        
        match self.read_file_internal(&req.path).await {
            Ok(data) => {
                log::info!("Read regional file success: {}", req.path);
                Ok(Response::new(ReadResponse {
                    content: data,
                    error: String::new(),
                }))
            }
            Err(e) => {
                log::error!("Read regional file failed: {} - {}", req.path, e);
                Ok(Response::new(ReadResponse {
                    content: vec![],
                    error: e.to_string(),
                }))
            }
        }
    }

    async fn delete_file(
        &self,
        request: Request<DeleteRequest>,
    ) -> Result<Response<DeleteResponse>, Status> {
        let req = request.into_inner();
        
        match self.delete_file_internal(&req.path).await {
            Ok(_) => {
                log::info!("Delete regional file success: {}", req.path);
                Ok(Response::new(DeleteResponse {
                    success: true,
                    error: String::new(),
                }))
            }
            Err(e) => {
                log::error!("Delete regional file failed: {} - {}", req.path, e);
                Ok(Response::new(DeleteResponse {
                    success: false,
                    error: e.to_string(),
                }))
            }
        }
    }

    async fn list_files(
        &self,
        request: Request<ListRequest>,
    ) -> Result<Response<ListResponse>, Status> {
        let req = request.into_inner();
        
        let metadata = self.metadata.lock().await;
        let files: Vec<String> = metadata
            .keys()
            .filter(|&k| k.starts_with(&req.dir))
            .cloned()
            .collect();
        
        log::info!("List regional files in {}: {} files found", req.dir, files.len());
        Ok(Response::new(ListResponse {
            files,
            error: String::new(),
        }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    log::info!("Starting Regional Metadata Node...");
    
    let addr = "127.0.0.1:50052".parse()?;
    let rmn = RegionalMetadataNode::new("./run/rmn".to_string(), "region_1".to_string());
    
    // 创建运行目录
    std::fs::create_dir_all("./run/rmn")?;
    
    let server = Server::builder()
        .add_service(MetadataServiceServer::new(rmn))
        .serve(addr);
    
    log::info!("Regional Metadata Node listening on {}", addr);
    
    server.await?;
    
    Ok(())
}