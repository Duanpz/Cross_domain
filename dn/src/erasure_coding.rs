use reed_solomon_erasure::galois_8::Field;
use reed_solomon_erasure::ReedSolomon;
use std::io::{Result, Error, ErrorKind};

pub struct ErasureCoder {
    rs: ReedSolomon<Field>,
    data_shards: usize,
    parity_shards: usize,
}

impl ErasureCoder {
    pub fn new(data_shards: usize, parity_shards: usize) -> Self {
        let rs = ReedSolomon::new(data_shards, parity_shards).unwrap();
        Self {
            rs,
            data_shards,
            parity_shards,
        }
    }

    pub fn encode(&self, data: &[u8]) -> Result<Vec<Vec<u8>>> {
        let chunks = self.split_into_chunks(data, self.data_shards);
        let mut shards = vec![vec![0u8; chunks[0].len()]; self.data_shards + self.parity_shards];
        
        // 填充数据块
        for i in 0..self.data_shards {
            shards[i] = chunks[i].clone();
        }
        
        // 编码
        self.rs.encode(&mut shards).map_err(|_| {
            Error::new(ErrorKind::Other, "Encoding failed")
        })?;
        
        Ok(shards)
    }

    pub fn decode(&self, shards: &[Vec<u8>], data_indices: &[usize]) -> Result<Vec<u8>> {
        let mut decoded_shards = shards.to_vec();
        
        // 解码
        self.rs.recover(&mut decoded_shards, data_indices).map_err(|_| {
            Error::new(ErrorKind::Other, "Decoding failed")
        })?;
        
        // 合并数据
        let mut result = Vec::new();
        for i in 0..self.data_shards {
            result.extend_from_slice(&decoded_shards[i]);
        }
        
        Ok(result)
    }

    fn split_into_chunks(&self, data: &[u8], num_chunks: usize) -> Vec<Vec<u8>> {
        let chunk_size = (data.len() + num_chunks - 1) / num_chunks;
        let mut chunks = Vec::with_capacity(num_chunks);
        
        for i in 0..num_chunks {
            let start = i * chunk_size;
            let end = (start + chunk_size).min(data.len());
            chunks.push(data[start..end].to_vec());
        }
        
        chunks
    }
}