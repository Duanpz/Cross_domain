use sdk::ChuanShiClient;
use std::collections::HashMap;
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting SDK benchmark client...");
    
    let mut client = ChuanShiClient::new().await?;
    
    // 性能测试
    let test_size = 1024 * 1024; // 1MB
    let test_data = vec![b'A'; test_size];
    
    println!("Testing with {} KB data", test_size / 1024);
    
    // 写入性能测试
    let start_time = Instant::now();
    let mut total_write_time = std::time::Duration::new(0, 0);
    
    for i in 0..10 {
        let filename = format!("/benchmark/test_{}.dat", i);
        let write_start = Instant::now();
        let success = client.create_file(&filename, test_data.clone()).await?;
        total_write_time += write_start.elapsed();
        println!("Write {}: {} ms", i, write_start.elapsed().as_millis());
    }
    
    let avg_write_time = total_write_time / 10;
    println!("Average write time: {} ms", avg_write_time.as_millis());
    
    // 读取性能测试
    let start_time = Instant::now();
    let mut total_read_time = std::time::Duration::new(0, 0);
    
    for i in 0..10 {
        let filename = format!("/benchmark/test_{}.dat", i);
        let read_start = Instant::now();
        let data = client.read_file(&filename).await?;
        total_read_time += read_start.elapsed();
        println!("Read {}: {} ms ({} bytes)", i, read_start.elapsed().as_millis(), data.len());
    }
    
    let avg_read_time = total_read_time / 10;
    println!("Average read time: {} ms", avg_read_time.as_millis());
    
    // 清理测试文件
    for i in 0..10 {
        let filename = format!("/benchmark/test_{}.dat", i);
        client.delete_file(&filename, true).await?;
    }
    
    println!("Benchmark completed in {} ms", start_time.elapsed().as_millis());
    
    Ok(())
}