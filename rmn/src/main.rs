use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use tonic::{transport::Server, Request, Response, Status};

mod pb {
    include!(concat!(env!("OUT_DIR"), "/chuangshi.rs"));
}
use pb::{MetadataServiceServer, CreateRequest, CreateResponse, ReadRequest, ReadResponse, DeleteRequest, DeleteResponse, ListRequest, ListResponse, GetAttributesRequest, GetAttributesResponse, UpdateMetadataRequest, UpdateMetadataResponse};

#[derive(Debug)]
struct MetadataService;

#[tonic::async_trait]
impl MetadataServiceServer for MetadataService {
    async fn create_file(&self, _request: Request<CreateRequest>) -> Result<Response<CreateResponse>, Status> {
        println!("RMN received create request");
        Ok(Response::new(CreateResponse { success: true }))
    }

    async fn read_file(&self, _request: Request<ReadRequest>) -> Result<Response<ReadResponse>, Status> {
        println!("RMN received read request");
        Ok(Response::new(ReadResponse { data: vec![] }))
    }

    async fn delete_file(&self, _request: Request<DeleteRequest>) -> Result<Response<DeleteResponse>, Status> {
        println!("RMN received delete request");
        Ok(Response::new(DeleteResponse { success: true, message: "File deleted".to_string() }))
    }

    async fn list_directory(&self, _request: Request<ListRequest>) -> Result<Response<ListResponse>, Status> {
        println!("RMN received list request");
        let files = vec![];
        let directories = vec![];
        Ok(Response::new(ListResponse { 
            files, 
            directories 
        }))
    }

    async fn get_file_attributes(&self, _request: Request<GetAttributesRequest>) -> Result<Response<GetAttributesResponse>, Status> {
        println!("RMN received get attributes request");
        Ok(Response::new(GetAttributesResponse { 
            file_info: None, 
            exists: false 
        }))
    }

    async fn update_file_metadata(&self, _request: Request<UpdateMetadataRequest>) -> Result<Response<UpdateMetadataResponse>, Status> {
        println!("RMN received update metadata request");
        Ok(Response::new(UpdateMetadataResponse { 
            success: true, 
            message: "Metadata updated".to_string() 
        }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 50052));
    let service = MetadataService;
    let server = Server::builder()
        .add_service(MetadataServiceServer::new(service))
        .serve(addr);

    println!("RMN listening on http://{}", addr);

    server.await?;
    Ok(())
}