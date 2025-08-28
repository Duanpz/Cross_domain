fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=../proto/chuangshi.proto");

    tonic_build::configure()
        .build_client(true)
        .build_server(true)
        // 让 tonic 自动给消息加常用派生，别再手动加
        .compile(&["../proto/chuangshi.proto"], &["../proto"])?;

    Ok(())
}
