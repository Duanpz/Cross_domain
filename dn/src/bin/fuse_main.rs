use std::thread;
use std::time::Duration;
use log::info;

mod fuse_server;
use fuse_server::FuseFileSystem;

fn main() {
    env_logger::init();
    
    info!("Starting FUSE filesystem...");
    
    // 启动 FUSE 服务
    let fuse_fs = FuseFileSystem::new();
    thread::spawn(move || {
        fuser::mount2(fuse_fs, "/tmp/chuangshi_fuse", &["ro", "noatime"]).unwrap();
    });

    // 保持主线程运行
    loop {
        thread::sleep(Duration::from_secs(1));
    }
}