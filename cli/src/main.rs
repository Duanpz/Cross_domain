use clap::Parser;
use sdk::*;

#[derive(Parser)]
struct Cli {
    #[clap(subcommand)]
    command: Command,
}

#[derive(clap::Subcommand)]
enum Command {
    Write { path: String, size: u64, owner: String },
    Read { path: String },
    Delete { path: String },
    List { path: String },
    Benchmark { 
        #[clap(long, default_value = "1000")]
        num_files: usize,
        #[clap(long, default_value = "1024")]
        file_size: usize,
        #[clap(long, default_value = "10")]
        concurrent_requests: usize,
        #[clap(long, default_value = "30")]
        duration_seconds: u64,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    
    let cli = Cli::parse();

    let mut client = ChuanShiClient::new().await?;

    match cli.command {
        Command::Write { path, size, owner } => {
            match client.create_file(&path, size, &owner).await {
                Ok(success) => {
                    if success {
                        println!("File written to {}", path);
                    } else {
                        println!("Failed to write file to {}", path);
                    }
                }
                Err(e) => {
                    eprintln!("Error writing file: {}", e);
                }
            }
        }
        Command::Read { path } => {
            match client.read_file(&path).await {
                Ok(data) => {
                    println!("Content: {:?}", String::from_utf8_lossy(&data));
                }
                Err(e) => {
                    eprintln!("Error reading file: {}", e);
                }
            }
        }
        Command::Delete { path } => {
            match client.delete_file(&path).await {
                Ok(success) => {
                    if success {
                        println!("File deleted");
                    } else {
                        println!("Failed to delete file");
                    }
                }
                Err(e) => {
                    eprintln!("Error deleting file: {}", e);
                }
            }
        }
        Command::List { path } => {
            match client.list_files(&path).await {
                Ok(files) => {
                    println!("Files in {}: {:?}", path, files);
                }
                Err(e) => {
                    eprintln!("Error listing files: {}", e);
                }
            }
        }
        Command::Benchmark { 
            num_files, 
            file_size, 
            concurrent_requests, 
            duration_seconds 
        } => {
            println!("Benchmark functionality would be implemented in benchmark module...");
        }
    }

    Ok(())
}