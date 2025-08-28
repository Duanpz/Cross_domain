use std::net::SocketAddr;
use std::sync::Arc;

use tokio::net::TcpListener;
use tonic::{transport::Server, Request, Response, Status};

mod pb {
    include!(concat!(env!("OUT_DIR"), "/chuangshi.rs"));
}
use pb::{MetadataServiceServer, CreateRequest, CreateResponse, ReadRequest, ReadResponse};

mod storage;
use storage::StorageManager;

#[derive(Debug)]
struct MetadataService {
    storage_manager: Arc<StorageManager>,
}

#[tonic::async_trait]
impl MetadataServiceServer for MetadataService {
    async fn create_file(&self, request: Request<CreateRequest>) -> Result<Response<CreateResponse>, Status> {
        let req = request.get_ref();
        println!("DN received create request for {:?}", req.path);
        
        // 存储文件数据
        if let Err(e) = self.storage_manager.write_file(&req.path, &req.data) {
            eprintln!("Failed to write file: {}", e);
            return Err(Status::internal("Failed to store file"));
        }
        
        Ok(Response::new(CreateResponse { success: true }))
    }

    async fn read_file(&self, request: Request<ReadRequest>) -> Result<Response<ReadResponse>, Status> {
        let req = request.get_ref();
        println!("DN received read request for {:?}", req.path);

        // 读取文件数据
        match self.storage_manager.read_file(&req.path) {
            Ok(data) => Ok(Response::new(ReadResponse { data })),
            Err(e) => {
                eprintln!("Failed to read file: {}", e);
                Err(Status::not_found("File not found"))
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    
    // 初始化存储管理器
    let storage_manager = Arc::new(StorageManager::new("./run/dn/data"));
    
    // 启动 gRPC 服务
    let addr = SocketAddr::from(([127, 0, 0, 1], 50053));
    let service = MetadataService {
        storage_manager,
    };
    let server = Server::builder()
        .add_service(MetadataServiceServer::new(service))
        .serve(addr);

    println!("DN listening on http://{}", addr);

    server.await?;
    Ok(())
}