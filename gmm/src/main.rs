use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use tonic::{transport::Server, Request, Response, Status};

mod pb {
    include!(concat!(env!("OUT_DIR"), "/chuangshi.rs"));
}
use pb::{MetadataServiceServer, CreateRequest, CreateResponse, ReadRequest, ReadResponse, DeleteRequest, DeleteResponse, ListRequest, ListResponse, GetAttributesRequest, GetAttributesResponse, UpdateMetadataRequest, UpdateMetadataResponse};

mod metadata;
use metadata::{MetadataManager, FileMeta};

#[derive(Debug)]
struct MetadataService {
    metadata_manager: Arc<MetadataManager>,
}

#[tonic::async_trait]
impl MetadataServiceServer for MetadataService {
    async fn create_file(&self, request: Request<CreateRequest>) -> Result<Response<CreateResponse>, Status> {
        let req = request.get_ref();
        println!("GMM received create request for {:?}", req.path);

        let meta = FileMeta {
            path: req.path.clone(),
            size: req.data.len() as u64,
            chunks: vec![],
            created_at: 0,
            updated_at: 0,
            node_id: req.node_id.clone(),
            replicas: req.replicas.clone(),
        };

        let success = self.metadata_manager.create_file(&req.path, meta);
        
        Ok(Response::new(CreateResponse { success }))
    }

    async fn read_file(&self, request: Request<ReadRequest>) -> Result<Response<ReadResponse>, Status> {
        let req = request.get_ref();
        println!("GMM received read request for {:?}", req.path);

        // 获取文件元数据
        let meta = self.metadata_manager.get_file(&req.path);
        if meta.is_none() {
            return Err(Status::not_found("File not found"));
        }

        // 这里可以返回实际数据（暂时返回空）
        Ok(Response::new(ReadResponse { data: vec![] }))
    }

    async fn delete_file(&self, request: Request<DeleteRequest>) -> Result<Response<DeleteResponse>, Status> {
        let req = request.get_ref();
        println!("GMM received delete request for {:?}", req.path);

        let success = self.metadata_manager.delete_file(&req.path);
        
        Ok(Response::new(DeleteResponse { success, message: "File deleted".to_string() }))
    }

    async fn list_directory(&self, request: Request<ListRequest>) -> Result<Response<ListResponse>, Status> {
        let req = request.get_ref();
        println!("GMM received list request for {:?}", req.path);

        // 简单的目录列表实现
        let files = vec![];
        let directories = vec![];
        
        Ok(Response::new(ListResponse { 
            files, 
            directories 
        }))
    }

    async fn get_file_attributes(&self, request: Request<GetAttributesRequest>) -> Result<Response<GetAttributesResponse>, Status> {
        let req = request.get_ref();
        println!("GMM received get attributes request for {:?}", req.path);

        let meta = self.metadata_manager.get_file(&req.path);
        if let Some(file_meta) = meta {
            let file_info = FileInfo {
                path: file_meta.path,
                size: file_meta.size,
                created_at: file_meta.created_at,
                modified_at: file_meta.updated_at,
                file_id: "file_id_123".to_string(),
                node_id: file_meta.node_id,
                replicas: file_meta.replicas,
                metadata: req.metadata.clone(), // 这里应该从实际元数据中获取
                checksum: "checksum_123".to_string(),
            };
            Ok(Response::new(GetAttributesResponse { 
                file_info: Some(file_info), 
                exists: true 
            }))
        } else {
            Ok(Response::new(GetAttributesResponse { 
                file_info: None, 
                exists: false 
            }))
        }
    }

    async fn update_file_metadata(&self, request: Request<UpdateMetadataRequest>) -> Result<Response<UpdateMetadataResponse>, Status> {
        let req = request.get_ref();
        println!("GMM received update metadata request for {:?}", req.path);

        // 这里应该更新文件元数据
        let success = true;
        
        Ok(Response::new(UpdateMetadataResponse { 
            success, 
            message: "Metadata updated".to_string() 
        }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 50051));
    
    let metadata_manager = Arc::new(MetadataManager::new("./run/gmm/metadata.json"));
    
    let service = MetadataService {
        metadata_manager,
    };

    let server = Server::builder()
        .add_service(MetadataServiceServer::new(service))
        .serve(addr);

    println!("GMM listening on http://{}", addr);

    server.await?;
    Ok(())
}