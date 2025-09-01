pub mod proto {
    tonic::include_proto!("chuangshi"); // 与 proto 文件中的 package 名一致
}


use tonic::{transport::Channel, Request};
use proto::metadata_service_client::MetadataServiceClient;
use proto::data_node_service_client::DataNodeServiceClient;






use proto::{metadata_service_server::MetadataServiceServer, CreateRequest, ReadRequest, DeleteRequest, ListRequest};


#[derive(Clone)]
pub struct ChuanShiClient {
    metadata_client: MetadataServiceClient<Channel>,
    data_client: DataNodeServiceClient<Channel>,
}

impl ChuanShiClient {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let metadata_client = MetadataServiceClient::connect("http://127.0.0.1:50051").await?;
        let data_client = DataNodeServiceClient::connect("http://127.0.0.1:50053").await?;
        Ok(ChuanShiClient { metadata_client, data_client })
    }

    pub async fn create_file(&mut self, path: &str, size: u64, owner: &str) -> Result<bool, Box<dyn std::error::Error>> {
        log::info!("Creating file via SDK: {}", path);
        let req = proto::CreateRequest {
            path: path.to_string(),
            size,
            owner: owner.to_string(),
        };
        let res = self.metadata_client.create_file(Request::new(req)).await?;
        let response = res.into_inner();
        log::info!("Create file result: success={}", response.success);
        Ok(response.success)
    }

    pub async fn read_file(&mut self, path: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        log::info!("Reading file via SDK: {}", path);
        let req = proto::ReadRequest {
            path: path.to_string(),
        };
        let res = self.metadata_client.read_file(Request::new(req)).await?;
        let response = res.into_inner();
        if !response.error.is_empty() {
            return Err(response.error.into());
        }
        log::info!("Read file result: {} bytes", response.content.len());
        Ok(response.content)
    }

    pub async fn delete_file(&mut self, path: &str) -> Result<bool, Box<dyn std::error::Error>> {
        log::info!("Deleting file via SDK: {}", path);
        let req = proto::DeleteRequest {
            path: path.to_string(),
        };
        let res = self.metadata_client.delete_file(Request::new(req)).await?;
        let response = res.into_inner();
        log::info!("Delete file result: success={}", response.success);
        Ok(response.success)
    }

    pub async fn list_files(&mut self, path: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        log::info!("Listing files via SDK: {}", path);
        let req = proto::ListRequest {
            dir: path.to_string(),
        };
        let res = self.metadata_client.list_files(Request::new(req)).await?;
        let response = res.into_inner();
        if !response.error.is_empty() {
            return Err(response.error.into());
        }
        log::info!("List files result: {} files", response.files.len());
        Ok(response.files)
    }
}