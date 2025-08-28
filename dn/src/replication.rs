use std::collections::HashMap;
use std::sync::Arc;
use std::time::SystemTime;
use tokio::time::{sleep, Duration};
use tonic::transport::Channel;
use crate::pb::MetadataServiceClient;

#[derive(Debug)]
pub struct ReplicationManager {
    // 存储其他节点的客户端
    remote_clients: Arc<std::sync::Mutex<HashMap<String, MetadataServiceClient<Channel>>>>,
}

impl ReplicationManager {
    pub fn new() -> Self {
        Self {
            remote_clients: Arc::new(std::sync::Mutex::new(HashMap::new())),
        }
    }

    pub fn add_remote_node(&self, node_id: &str, address: &str) -> Result<(), Box<dyn std::error::Error>> {
        let channel = Channel::from_shared(address.to_string())?
            .connect()
            .await?;
        let client = MetadataServiceClient::new(channel);
        
        self.remote_clients.lock().unwrap().insert(
            node_id.to_string(),
            client
        );
        Ok(())
    }

    pub async fn replicate_file_to_node(
        &self,
        node_id: &str,
        path: &str,
        data: &[u8],
    ) -> Result<(), Box<dyn std::error::Error>> {
        let clients = self.remote_clients.lock().unwrap();
        if let Some(client) = clients.get(node_id) {
            // 这里应该调用远程的 create_file 方法
            // 暂时简化为打印日志
            println!("Replicating {} to node {}", path, node_id);
            Ok(())
        } else {
            Err("Node not found".into())
        }
    }
}