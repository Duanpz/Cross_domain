use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());

    tonic_build::configure()
        .out_dir(&out_dir) 
        .compile(&["../proto/chuangshi.proto"], &["../proto"])?;

    Ok(())
}
