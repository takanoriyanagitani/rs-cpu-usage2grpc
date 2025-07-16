use std::env;
use std::net::SocketAddr;

use rs_cpu_usage2grpc::{cpu_usage_svc_new, start_svc};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr: SocketAddr = env::var("CPU_USAGE_ADDR")
        .unwrap_or_else(|_| "[::1]:50051".to_string())
        .parse()?;

    let svc = cpu_usage_svc_new();

    println!("CpuUsage server listening on {}", addr);

    start_svc(svc, addr).await?;

    Ok(())
}
