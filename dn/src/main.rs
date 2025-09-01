use tokio::net::TcpListener;
use tonic::{transport::Server, Request, Response, Status};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use std::time::SystemTime;
use std::path::Path;
use std::fs;
use tokio::fs as async_fs;

mod erasure_coding;
mod proto {
    tonic::include_proto!("chuangshi");
}



use erasure_coding::ErasureCoder;
use proto::{ data_node_service_server::DataNodeServiceServer, StoreRequest, RetrieveRequest,StoreResponse,RetrieveResponse,ReadResponse,DeleteResponse,ListResponse};

pub struct DataNode {
    data_dir: String,
    erasure_coder: ErasureCoder,
    chunks: Arc<Mutex<HashMap<String, String>>>, // chunk_id -> shard_paths
}

impl DataNode {
    pub fn new(data_dir: String) -> Self {
        let coder = ErasureCoder::new(4, 2); // 4数据块 + 2校验块
        Self {
            data_dir,
            erasure_coder: coder,
            chunks: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    async fn store_chunk(&self, chunk_id: &str, data: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        log::info!("Storing chunk: {}", chunk_id);
        
        // 使用纠删码编码
        let shards = self.erasure_coder.encode(data);
        
        // 存储每个shard到本地文件系统
        let mut shard_paths = Vec::new();
        for (i, shard) in shards.iter().enumerate() {
            let shard_path = format!("{}/{}_shard_{}", self.data_dir, chunk_id, i);
            async_fs::write(&shard_path, shard).await?;
            shard_paths.push(shard_path);
        }
        
        // 记录元数据
        let mut chunks = self.chunks.lock().await;
        chunks.insert(chunk_id.to_string(), shard_paths.join(","));
        
        log::info!("Successfully stored chunk: {}", chunk_id);
        Ok(())
    }

    async fn retrieve_chunk(&self, chunk_id: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        log::info!("Retrieving chunk: {}", chunk_id);
        
        // 读取所有shard
        let mut shards = Vec::new();
        let mut available_indices = Vec::new();
        
        for i in 0..self.erasure_coder.total_shards() {
            let shard_path = format!("{}/{}_shard_{}", self.data_dir, chunk_id, i);
            if Path::new(&shard_path).exists() {
                let shard_data = async_fs::read(&shard_path).await?;
                shards.push(shard_data);
                available_indices.push(i);
            } else {
                shards.push(vec![]); // 空shard占位
            }
        }
        
        // 解码恢复数据
        // let data = self.erasure_coder.decode(&shards, &available_indices);
        let data = self.erasure_coder.decode(shards.clone(), &available_indices);
        log::info!("Successfully retrieved chunk: {}", chunk_id);
        Ok(data)



    }
}

#[tonic::async_trait]
impl proto::data_node_service_server::DataNodeService for DataNode {
    async fn store_chunk(
        &self,
        request: Request<StoreRequest>,
    ) -> Result<Response<StoreResponse>, Status> {
        let req = request.into_inner();
        match self.store_chunk(&req.chunk_id, &req.data).await {
            Ok(_) => Ok(Response::new(StoreResponse {
                success: true,
                error: String::new(),
            })),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    async fn retrieve_chunk(
        &self,
        request: Request<RetrieveRequest>,
    ) -> Result<Response<RetrieveResponse>, Status> {
        let req = request.into_inner();
        match self.retrieve_chunk(&req.chunk_id).await {
            Ok(data) => Ok(Response::new(RetrieveResponse {
                data,
                error: String::new(),
            })),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    log::info!("Starting Data Node...");
    
    let addr = "127.0.0.1:50053".parse()?;
    let data_node = DataNode::new("./run/dn/data".to_string());
    
    // 创建数据目录
    async_fs::create_dir_all("./run/dn/data").await?;
    
    let server = Server::builder()
        .add_service(DataNodeServiceServer::new(data_node))
        .serve(addr);
    
    log::info!("Data Node listening on {}", addr);
    
    server.await?;
    
    Ok(())
}