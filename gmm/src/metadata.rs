use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use parking_lot::RwLock;
use std::fs;
use std::path::Path;
use chrono::Utc;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileMeta {
    pub path: String,
    pub size: u64,
    pub chunks: Vec<String>, // chunk ids
    pub created_at: i64,
    pub updated_at: i64,
    pub node_id: String, // 所在节点ID
    pub replicas: Vec<String>, // 副本节点列表
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetadataStore {
    pub files: HashMap<String, FileMeta>,
    pub nodes: HashMap<String, NodeInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeInfo {
    pub id: String,
    pub address: String,
    pub last_heartbeat: i64,
    pub status: String,
}

impl Default for MetadataStore {
    fn default() -> Self {
        Self {
            files: HashMap::new(),
            nodes: HashMap::new(),
        }
    }
}

pub struct MetadataManager {
    store: Arc<RwLock<MetadataStore>>,
    storage_path: String,
}

impl MetadataManager {
    pub fn new(storage_path: &str) -> Self {
        let store = Arc::new(RwLock::new(Self::load_from_disk(storage_path)));
        Self {
            store,
            storage_path: storage_path.to_string(),
        }
    }

    fn load_from_disk(path: &str) -> MetadataStore {
        if let Ok(data) = fs::read_to_string(path) {
            if let Ok(store) = serde_json::from_str(&data) {
                return store;
            }
        }
        MetadataStore::default()
    }

    pub fn save_to_disk(&self) {
        let store = self.store.read();
        let json_data = serde_json::to_string_pretty(&*store).unwrap();
        fs::write(&self.storage_path, json_data).unwrap();
    }

    pub fn get_file(&self, path: &str) -> Option<FileMeta> {
        self.store.read().files.get(path).cloned()
    }

    pub fn create_file(&self, path: &str, mut meta: FileMeta) -> bool {
        let now = Utc::now().timestamp();
        meta.created_at = now;
        meta.updated_at = now;
        
        let mut store = self.store.write();
        store.files.insert(path.to_string(), meta);
        self.save_to_disk();
        true
    }

    pub fn update_file(&self, path: &str, mut meta: FileMeta) -> bool {
        let now = Utc::now().timestamp();
        meta.updated_at = now;
        
        let mut store = self.store.write();
        if store.files.contains_key(path) {
            store.files.insert(path.to_string(), meta);
            self.save_to_disk();
            true
        } else {
            false
        }
    }

    pub fn delete_file(&self, path: &str) -> bool {
        let mut store = self.store.write();
        let deleted = store.files.remove(path).is_some();
        self.save_to_disk();
        deleted
    }

    pub fn register_node(&self, node_id: &str, address: &str) -> bool {
        let now = Utc::now().timestamp();
        let node_info = NodeInfo {
            id: node_id.to_string(),
            address: address.to_string(),
            last_heartbeat: now,
            status: "online".to_string(),
        };
        
        let mut store = self.store.write();
        store.nodes.insert(node_id.to_string(), node_info);
        self.save_to_disk();
        true
    }

    pub fn heartbeat(&self, node_id: &str) -> bool {
        let now = Utc::now().timestamp();
        let mut store = self.store.write();
        if let Some(node) = store.nodes.get_mut(node_id) {
            node.last_heartbeat = now;
            self.save_to_disk();
            true
        } else {
            false
        }
    }

    pub fn get_files_by_node(&self, node_id: &str) -> Vec<FileMeta> {
        let store = self.store.read();
        store.files.values()
            .filter(|meta| meta.node_id == *node_id)
            .cloned()
            .collect()
    }
}