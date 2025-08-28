use sdk::ChuanShiClient;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting SDK test client...");
    
    let mut client = ChuanShiClient::new().await?;
    
    // 测试写入
    let test_data = b"Hello, World! This is a test file from SDK.";
    let success = client.create_file("/sdk/test.txt", test_data.to_vec()).await?;
    println!("Write result: {}", success);
    
    // 测试读取
    let data = client.read_file("/sdk/test.txt").await?;
    println!("Read data: {:?}", String::from_utf8_lossy(&data));
    
    // 测试文件大小
    let size = client.get_file_size("/sdk/test.txt").await?;
    println!("File size: {} bytes", size);
    
    // 测试文件存在性
    let exists = client.file_exists("/sdk/test.txt").await?;
    println!("File exists: {}", exists);
    
    // 测试删除
    let deleted = client.delete_file("/sdk/test.txt", true).await?;
    println!("Delete result: {}", deleted);
    
    // 测试元数据更新
    let mut metadata = HashMap::new();
    metadata.insert("author".to_string(), "SDK Test".to_string());
    metadata.insert("version".to_string(), "1.0".to_string());
    
    let updated = client.update_file_metadata("/sdk/test.txt", metadata).await?;
    println!("Metadata update result: {}", updated);
    
    println!("SDK test completed successfully!");
    
    Ok(())
}