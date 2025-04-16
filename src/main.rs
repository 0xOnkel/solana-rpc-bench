use solana_rpc_bench::{client::Client, setting::Settings};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    // this should help, but is not mandatory
    // user can provide the env vars in another way
    let _ = dotenvy::dotenv();

    let settings = Settings::new();
    let rpcs: Vec<Client> = settings.rpcs.into_iter().map(|rpc| rpc.into()).collect();

    for rpc in rpcs {
        rpc.test(3).await;
    }
}
