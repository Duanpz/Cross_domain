
// use reed_solomon_erasure::galois_8::ReedSolomon;
// // 如果确实需要 Code，在新版里它叫 galois_8::Field 或 galois_16::Field

// #[derive(Debug)]
// pub struct ErasureCoder {
//     data_shards: usize,
//     parity_shards: usize,
//     rs: reed_solomon_erasure::galois_8::ReedSolomon,
// }

// impl ErasureCoder {
//     pub fn new(data_shards: usize, parity_shards: usize) -> Self {
//         let rs = ReedSolomon::new(data_shards, parity_shards).unwrap();
//         Self {
//             data_shards,
//             parity_shards,
//             rs,
//         }
//     }

//     /// 将数据分块并编码为多个shard
//     pub fn encode(&self, data: &[u8]) -> Vec<Vec<u8>> {
//         let chunk_size = 1024 * 1024; // 1MB chunks
//         let chunks = self.chunk_data(data, chunk_size);
        
//         let total_shards = self.data_shards + self.parity_shards;
//         let mut all_shards = vec![vec![0u8; chunks[0].len()]; total_shards];
        
//         // 填充数据块
//         for (i, chunk) in chunks.iter().enumerate() {
//             all_shards[i].clone_from_slice(chunk);
//         }
        
//         // 编码生成校验块
//         self.rs.encode(&mut all_shards).unwrap();
        
//         all_shards
//     }

//     /// 从shards中恢复原始数据
//     pub fn decode(&self, shards: &[Vec<u8>], available_indices: &[usize]) -> Vec<u8> {
//         let mut working_shards = shards.to_vec();
//         let mut available = Vec::new();
        
//         // 标记可用的shard索引
//         for &idx in available_indices {
//             if idx < working_shards.len() {
//                 available.push(idx);
//             }
//         }
        
//         // 恢复数据
//         // self.rs.recover(&mut working_shards, &available).unwrap();
//         self.rs.reconstruct(&mut working_shards，).unwrap();
//         // 拼接恢复的数据
//         let mut result = Vec::new();
//         for i in 0..self.data_shards {
//             if i < working_shards.len() {
//                 result.extend_from_slice(&working_shards[i]);
//             }
//         }
//         result
//     }

//     /// 数据分块
//     fn chunk_data(&self, data: &[u8], chunk_size: usize) -> Vec<Vec<u8>> {
//         let mut chunks = Vec::new();
//         for chunk in data.chunks(chunk_size) {
//             chunks.push(chunk.to_vec());
//         }
//         chunks
//     }

//     /// 获取shard数量
//     pub fn total_shards(&self) -> usize {
//         self.data_shards + self.parity_shards
//     }

//     /// 获取数据块数量
//     pub fn data_shards(&self) -> usize {
//         self.data_shards
//     }
// }
use reed_solomon_erasure::galois_8::ReedSolomon;

#[derive(Debug)]
pub struct ErasureCoder {
    data_shards: usize,
    parity_shards: usize,
    rs: ReedSolomon,
}

impl ErasureCoder {
    pub fn new(data_shards: usize, parity_shards: usize) -> Self {
        let rs = ReedSolomon::new(data_shards, parity_shards).unwrap();
        Self {
            data_shards,
            parity_shards,
            rs,
        }
    }

   pub fn encode(&self, data: &[u8]) -> Vec<Vec<u8>> {
        let chunk_size = 1024 * 1024;
        let chunks = self.chunk_data(data, chunk_size);

        let shard_len = chunks[0].len();
        let mut all = vec![vec![0u8; shard_len]; self.total_shards()];

        // 复制数据块
        for (i, chunk) in chunks.iter().enumerate().take(self.data_shards) {
            all[i].copy_from_slice(chunk);
        }

        // 生成校验块
        let (data, parity) = all.split_at_mut(self.data_shards);
        self.rs.encode_sep(data, parity).unwrap();
        all
    }

    pub fn decode(&self, shards: Vec<Vec<u8>>, available: &[usize]) -> Vec<u8> {
    // 1. 转为 Vec<Option<Vec<u8>>>，缺失块填 None
    let mut opt_shards: Vec<Option<Vec<u8>>> = shards
        .into_iter()
        .enumerate()
        .map(|(i, buf)| {
            if available.contains(&i) {
                Some(buf)
            } else {
                None
            }
        })
        .collect();

    // 2. 重建
    self.rs.reconstruct(&mut opt_shards).unwrap();

    // 3. 把数据块拼回
    opt_shards
        .into_iter()
        .take(self.data_shards)
        .flatten()
        .flatten()
        .collect()
}



    /* ---------- 其余辅助函数保持不变 ---------- */
    fn chunk_data(&self, data: &[u8], chunk_size: usize) -> Vec<Vec<u8>> {
        data.chunks(chunk_size).map(|c| c.to_vec()).collect()
    }

    pub fn total_shards(&self) -> usize { self.data_shards + self.parity_shards }
    pub fn data_shards(&self) -> usize { self.data_shards }
}
