use solana_client::nonblocking::rpc_client::RpcClient;
use tokio::time::Instant;

use crate::setting::SettingClient;

pub struct Client {
    label: String,
    rpc: RpcClient,
}

impl From<SettingClient> for Client {
    fn from(value: SettingClient) -> Self {
        Self {
            label: value.label,
            rpc: RpcClient::new(value.url),
        }
    }
}

impl Client {
    pub async fn test(&self) {
        let start = Instant::now();
        let slot = self.rpc.get_slot().await.unwrap();
        println!(
            "{}, Result: {slot:?}, Duration: {:#?}",
            self.label,
            start.elapsed()
        );
    }
}
