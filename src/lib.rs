use core::net::SocketAddr;
use core::time::Duration;

use std::future::Future;

use std::io;

use psutil::cpu::CpuPercentCollector;

use tonic::Request;
use tonic::Response;
use tonic::Status;

pub mod cpu_usage {
    pub mod v1 {
        tonic::include_proto!("cpu_usage.v1");
    }
}

use crate::cpu_usage::v1::CpuUsageRequest;
use crate::cpu_usage::v1::CpuUsageResponse;
use crate::cpu_usage::v1::CpuUsageStat;
use crate::cpu_usage::v1::cpu_usage_server::CpuUsage;
use crate::cpu_usage::v1::cpu_usage_server::CpuUsageServer;

async fn _get_raw_cpu_percentages<F, W>(
    wait_duration: Duration,
    sleep_async: F,
) -> Result<Vec<f32>, io::Error>
where
    F: Fn(Duration) -> W,
    W: Future<Output = ()>,
{
    let mut cpu_percent_collector =
        CpuPercentCollector::new().map_err(|e| io::Error::other(e.to_string()))?;

    // The first call returns None, but it initializes the collector.
    cpu_percent_collector
        .cpu_percent_percpu()
        .map_err(|e| io::Error::other(e.to_string()))?;

    sleep_async(wait_duration).await;

    cpu_percent_collector
        .cpu_percent_percpu()
        .map_err(|e| io::Error::other(e.to_string()))
}

fn _convert_percentages_to_stats(percentages: Vec<f32>) -> Vec<CpuUsageStat> {
    percentages
        .into_iter()
        .enumerate()
        .map(|(i, p)| CpuUsageStat {
            cpuid: i as u32,
            usage: p,
        })
        .collect()
}

pub async fn get_cpu_percent<F, W>(
    wait_duration: Duration,
    sleep_async: F,
) -> Result<Vec<CpuUsageStat>, io::Error>
where
    F: Fn(Duration) -> W,
    W: Future<Output = ()>,
{
    let percentages = _get_raw_cpu_percentages(wait_duration, sleep_async).await?;
    Ok(_convert_percentages_to_stats(percentages))
}

#[derive(Clone)]
struct CpuUsageSvc {}

#[tonic::async_trait]
impl CpuUsage for CpuUsageSvc {
    async fn get_cpu_usage(
        &self,
        req: Request<CpuUsageRequest>,
    ) -> Result<Response<CpuUsageResponse>, Status> {
        let wait_duration_ms = req.into_inner().wait_duration_ms;
        let wait_duration = Duration::from_millis(wait_duration_ms as u64);

        let stats = get_cpu_percent(wait_duration, tokio::time::sleep)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(CpuUsageResponse { stats }))
    }
}

pub fn cpu_usage_svc_new() -> impl CpuUsage + Clone {
    CpuUsageSvc {}
}

pub async fn start_svc<S>(svc: S, addr: SocketAddr) -> Result<(), io::Error>
where
    S: CpuUsage + Clone,
{
    tonic::transport::Server::builder()
        .add_service(CpuUsageServer::new(svc))
        .serve(addr)
        .await
        .map_err(io::Error::other)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    async fn mock_sleep(duration: Duration) {
        tokio::time::sleep(duration).await;
    }

    #[tokio::test]
    async fn test_get_cpu_percent() {
        let stats = get_cpu_percent(Duration::from_millis(100), mock_sleep)
            .await
            .unwrap();
        assert!(!stats.is_empty());
        for stat in stats {
            assert!(stat.usage >= 0.0);
            assert!(stat.usage <= 100.0);
        }
    }
}
