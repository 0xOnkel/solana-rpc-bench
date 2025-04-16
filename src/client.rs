use std::{ops::Div, time::Duration};

use solana_client::{client_error::Result as ClientResult, nonblocking::rpc_client::RpcClient};
use tokio::time::{Instant, sleep};
use tracing::error;

use crate::{Result, setting::SettingClient};

pub struct Client {
    label: String,
    pub rpc: RpcClient,
}

impl From<SettingClient> for Client {
    fn from(value: SettingClient) -> Self {
        Self {
            label: value.label,
            rpc: RpcClient::new(value.url),
        }
    }
}

struct TestResult {
    best: Duration,
    worst: Duration,
    total: Duration,
    count: u32,
}

impl TestResult {
    fn log(&self, label: &str, call: &str) {
        println!(
            "{} - {}\n\tAvg. Duration: {:#?}\n\tBest Duration: {:?}\n\tWorst Duration: {:?}",
            label,
            call,
            self.total.div(self.count),
            self.best,
            self.worst,
        );
    }
}

impl Client {
    pub async fn test(&self, count: u32) {
        self.run_test(|| self.rpc.get_slot(), count)
            .await
            .log(self.label.as_str(), "get_slot");
    }

    async fn run_test<F, Fut, T>(&self, mut f: F, count: u32) -> TestResult
    where
        F: FnMut() -> Fut,
        Fut: Future<Output = ClientResult<T>>,
    {
        let mut best = Duration::MAX;
        let mut worst = Duration::ZERO;
        let mut total = Duration::ZERO;

        for _ in 0..count {
            match self.internal_test(f()).await {
                Ok(duration) => {
                    if duration < best {
                        best = duration
                    }
                    if duration > worst {
                        worst = duration
                    }
                    total += duration;
                }
                Err(e) => {
                    error!("{e}");
                }
            }

            // we backoff a bit, this is not an rpc load test
            sleep(Duration::from_millis(500)).await;
        }

        TestResult {
            best,
            worst,
            total,
            count,
        }
    }

    // this is our internal test which allows us to reuse the Duration return for the different rpc
    // calls
    async fn internal_test<T>(&self, f: impl Future<Output = ClientResult<T>>) -> Result<Duration> {
        let start = Instant::now();
        let _ = f.await?;
        Ok(start.elapsed())
    }
}
