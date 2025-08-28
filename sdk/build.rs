fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=../proto/chuangshi.proto");

    tonic_build::configure()
        .build_client(true)
        .build_server(true)
        // 让 tonic 自动给消息加常用派生，别再手动加
        .compile(&["../proto/chuangshi.proto"], &["../proto"])?;

    Ok(())
}


mod chuangshi {
    tonic::include_proto!("chuangshi");
}

// 按需导出
pub use chuangshi::{
    metadata_service_client::MetadataServiceClient,
    CreateRequest,
    CreateResponse,
    ReadRequest,
    ReadResponse,
};