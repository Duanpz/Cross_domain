use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use parking_lot::RwLock;
use std::io::Write;
use reed_solomon_erasure::galois_8::Field;
use reed_solomon_erasure::ReedSolomon;

pub struct StorageManager {
    base_path: PathBuf,
    file_locks: Arc<RwLock<HashMap<String, ()>>>,
    erasure_coder: Arc<RwLock<Option<ReedSolomon<Field>>>>,
}

impl StorageManager {
    pub fn new(base_path: &str) -> Self {
        let path = PathBuf::from(base_path);
        if !path.exists() {
            fs::create_dir_all(&path).unwrap();
        }
        
        Self {
            base_path: path,
            file_locks: Arc::new(RwLock::new(HashMap::new())),
            erasure_coder: Arc::new(RwLock::new(None)),
        }
    }

    pub fn set_erasure_coding(&self, data_shards: usize, parity_shards: usize) {
        let rs = ReedSolomon::new(data_shards, parity_shards).unwrap();
        *self.erasure_coder.write() = Some(rs);
    }

    pub fn write_file(&self, path: &str, data: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        let full_path = self.base_path.join(path);
        
        // 确保父目录存在
        if let Some(parent) = full_path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        // 如果启用了纠删码，则进行编码
        if let Some(ref coder) = *self.erasure_coder.read() {
            let shards = coder.encode(data)?;
            // 将分片保存到不同的文件中
            for (i, shard) in shards.iter().enumerate() {
                let shard_path = full_path.with_extension(format!(".shard{}", i));
                let mut file = fs::File::create(&shard_path)?;
                file.write_all(shard)?;
                file.flush()?;
            }
        } else {
            // 直接写入文件
            let mut file = fs::File::create(full_path)?;
            file.write_all(data)?;
            file.flush()?;
        }
        
        Ok(())
    }

    pub fn read_file(&self, path: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let full_path = self.base_path.join(path);
        
        // 检查是否有分片
        if let Some(coder) = &*self.erasure_coder.read() {
            // 读取所有分片
            let mut shards = Vec::new();
            let mut data_indices = Vec::new();
            
            for i in 0..coder.data_shards() {
                let shard_path = full_path.with_extension(format!(".shard{}", i));
                if shard_path.exists() {
                    let data = fs::read(&shard_path)?;
                    shards.push(data);
                    data_indices.push(i);
                }
            }
            
            // 解码
            if !shards.is_empty() {
                return Ok(coder.decode(&shards, &data_indices)?);
            }
        }
        
        // 直接读取文件
        let data = fs::read(full_path)?;
        Ok(data)
    }

    pub fn delete_file(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let full_path = self.base_path.join(path);
        if full_path.exists() {
            fs::remove_file(full_path)?;
        }
        Ok(())
    }

    pub fn file_exists(&self, path: &str) -> bool {
        let full_path = self.base_path.join(path);
        full_path.exists()
    }

    pub fn get_file_size(&self, path: &str) -> Result<u64, Box<dyn std::error::Error>> {
        let full_path = self.base_path.join(path);
        let metadata = fs::metadata(full_path)?;
        Ok(metadata.len())
    }
}