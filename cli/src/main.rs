use clap::Parser;
use std::collections::HashMap;
use std::path::Path;
use std::fs;
use std::io::{self, Write};

// 使用 SDK 而不是直接使用 gRPC
use sdk::ChuanShiClient;

#[derive(Parser)]
#[clap(name = "chuangshi", version = "0.1.0", about = "创世 NDFS 命令行客户端")]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
    
    /// 配置文件路径
    #[clap(long, default_value = "~/.config/chuangshi/config.toml")]
    config: String,
    
    /// 日志级别
    #[clap(long, default_value = "info")]
    log_level: String,
}

#[derive(clap::Subcommand)]
enum Commands {
    /// 创建文件
    Put {
        /// 本地文件路径
        source: String,
        /// 远程文件路径
        destination: String,
    },
    
    /// 读取文件
    Get {
        /// 远程文件路径
        source: String,
        /// 本地文件路径（可选）
        destination: Option<String>,
    },
    
    /// 删除文件
    Delete {
        /// 文件路径
        path: String,
    },
    
    /// 列出目录
    List {
        /// 目录路径
        path: Option<String>,
        /// 递归列出
        #[clap(long)]
        recursive: bool,
    },
    
    /// 显示文件信息
    Stat {
        /// 文件路径
        path: String,
    },
    
    /// 创建目录
    Mkdir {
        /// 目录路径
        path: String,
    },
    
    /// 复制文件
    Cp {
        /// 源文件路径
        source: String,
        /// 目标文件路径
        destination: String,
    },
    
    /// 移动文件
    Mv {
        /// 源文件路径
        source: String,
        /// 目标文件路径
        destination: String,
    },
    
    /// 设置元数据
    SetMeta {
        /// 文件路径
        path: String,
        /// 元数据键值对 (key=value)
        metadata: Vec<String>,
    },
    
    /// 获取元数据
    GetMeta {
        /// 文件路径
        path: String,
    },
    
    /// 配置管理
    Config {
        #[clap(subcommand)]
        config_cmd: ConfigCommands,
    },
}

#[derive(clap::Subcommand)]
enum ConfigCommands {
    /// 显示配置
    Show,
    /// 设置配置项
    Set {
        /// 配置项名称
        key: String,
        /// 配置值
        value: String,
    },
    /// 重置配置
    Reset,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    
    let cli = Cli::parse();
    
    // 使用 SDK 客户端
    let mut client = ChuanShiClient::new().await?;
    
    match &cli.command {
        Commands::Put { source, destination } => {
            put_file(&mut client, source, destination).await?;
        }
        Commands::Get { source, destination } => {
            get_file(&mut client, source, destination.as_deref()).await?;
        }
        Commands::Delete { path } => {
            delete_file(&mut client, path).await?;
        }
        Commands::List { path, recursive } => {
            list_directory(&mut client, path.as_deref(), *recursive).await?;
        }
        Commands::Stat { path } => {
            stat_file(&mut client, path).await?;
        }
        Commands::Mkdir { path } => {
            mkdir(&mut client, path).await?;
        }
        Commands::Cp { source, destination } => {
            copy_file(&mut client, source, destination).await?;
        }
        Commands::Mv { source, destination } => {
            move_file(&mut client, source, destination).await?;
        }
        Commands::SetMeta { path, metadata } => {
            set_metadata(&mut client, path, metadata).await?;
        }
        Commands::GetMeta { path } => {
            get_metadata(&mut client, path).await?;
        }
        Commands::Config { config_cmd } => {
            handle_config_command(config_cmd, &cli.config)?;
        }
    }
    
    Ok(())
}

async fn put_file(
    client: &mut ChuanShiClient,
    source: &str,
    destination: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // 读取本地文件
    let data = fs::read(source)?;
    
    // 使用 SDK 方法
    let success = client.create_file(destination, data).await?;
    
    if success {
        println!("✓ 文件已上传到: {}", destination);
    } else {
        eprintln!("✗ 文件上传失败");
    }
    
    Ok(())
}

async fn get_file(
    client: &mut ChuanShiClient,
    source: &str,
    destination: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    // 使用 SDK 方法
    let resp = client.read_file(source).await?;
    
    match destination {
        Some(dest) => {
            // 保存到本地文件
            fs::write(dest, &resp)?;
            println!("✓ 文件已下载到: {}", dest);
        }
        None => {
            // 输出到标准输出
            io::stdout().write_all(&resp)?;
            println!();
        }
    }
    
    Ok(())
}

async fn delete_file(
    client: &mut ChuanShiClient,
    path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // 使用 SDK 方法
    let success = client.delete_file(path, true).await?;
    
    if success {
        println!("✓ 文件已删除: {}", path);
    } else {
        eprintln!("✗ 文件删除失败: {}", path);
    }
    
    Ok(())
}

async fn list_directory(
    client: &mut ChuanShiClient,
    path: Option<&str>,
    recursive: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    // 使用 SDK 方法
    let resp = client.list_directory(path.unwrap_or("/"), recursive).await?;
    
    println!("目录内容: {}", path.unwrap_or("/"));
    println!("文件数量: {}", resp.files.len());
    println!("目录数量: {}", resp.directories.len());
    
    // 显示文件列表
    for file in &resp.files {
        println!("  📄 {} ({} bytes)", file.path, file.size);
    }
    
    // 显示目录列表
    for dir in &resp.directories {
        println!("  📁 {} ({}/{} items)", dir.path, dir.file_count, dir.total_size);
    }
    
    Ok(())
}

async fn stat_file(
    client: &mut ChuanShiClient,
    path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // 使用 SDK 方法
    let resp = client.get_file_attributes(path).await?;
    
    if let Some(file_info) = resp.file_info {
        println!("文件信息:");
        println!("  路径: {}", file_info.path);
        println!("  大小: {} 字节", file_info.size);
        println!("  节点: {}", file_info.node_id);
        println!("  副本数: {}", file_info.replicas.len());
        println!("  创建时间: {}", file_info.created_at);
        println!("  修改时间: {}", file_info.modified_at);
        println!("  文件ID: {}", file_info.file_id);
        println!("  校验和: {}", file_info.checksum);
    } else {
        println!("文件不存在: {}", path);
    }
    
    Ok(())
}

async fn mkdir(
    client: &mut ChuanShiClient,
    path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // 使用 SDK 方法
    let success = client.create_file(path, vec![]).await?;
    
    if success {
        println!("✓ 目录已创建: {}", path);
    } else {
        eprintln!("✗ 目录创建失败: {}", path);
    }
    
    Ok(())
}

async fn copy_file(
    client: &mut ChuanShiClient,
    source: &str,
    destination: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // 读取源文件
    let data = client.read_file(source).await?;
    
    // 创建目标文件
    let success = client.create_file(destination, data).await?;
    
    if success {
        println!("✓ 文件已复制: {} -> {}", source, destination);
    } else {
        eprintln!("✗ 文件复制失败: {} -> {}", source, destination);
    }
    
    Ok(())
}

async fn move_file(
    client: &mut ChuanShiClient,
    source: &str,
    destination: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // 先复制文件
    copy_file(client, source, destination).await?;
    
    // 再删除源文件
    delete_file(client, source).await?;
    
    println!("✓ 文件已移动: {} -> {}", source, destination);
    
    Ok(())
}

async fn set_metadata(
    client: &mut ChuanShiClient,
    path: &str,
    metadata_list: &[String],
) -> Result<(), Box<dyn std::error::Error>> {
    // 解析元数据键值对
    let mut metadata = HashMap::new();
    for item in metadata_list {
        if let Some(pos) = item.find('=') {
            let key = item[..pos].to_string();
            let value = item[pos + 1..].to_string();
            metadata.insert(key, value);
        } else {
            eprintln!("警告: 无效的元数据格式: {}", item);
        }
    }
    
    // 使用 SDK 方法
    let success = client.update_file_metadata(path, metadata).await?;
    
    if success {
        println!("✓ 元数据已设置: {}", path);
    } else {
        eprintln!("✗ 元数据设置失败: {}", path);
    }
    
    Ok(())
}

async fn get_metadata(
    client: &mut ChuanShiClient,
    path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // 使用 SDK 方法
    let resp = client.get_file_attributes(path).await?;
    
    if let Some(file_info) = resp.file_info {
        println!("文件元数据:");
        println!("  路径: {}", file_info.path);
        println!("  大小: {} 字节", file_info.size);
        println!("  节点: {}", file_info.node_id);
        println!("  副本数: {}", file_info.replicas.len());
        println!("  创建时间: {}", file_info.created_at);
        println!("  修改时间: {}", file_info.modified_at);
        println!("  文件ID: {}", file_info.file_id);
        println!("  校验和: {}", file_info.checksum);
    } else {
        println!("文件不存在: {}", path);
    }
    
    Ok(())
}

fn handle_config_command(
    cmd: &ConfigCommands,
    config_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // 配置管理逻辑...
    Ok(())
}