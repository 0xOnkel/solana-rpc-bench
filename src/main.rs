use solana_rpc_bench::{client::Client, setting::Settings};
use tabled::{
    builder::Builder,
    settings::{Alignment, Style, object::Columns},
};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    // this should help, but is not mandatory
    // user can provide the env vars in another way
    let _ = dotenvy::dotenv();

    let settings = Settings::new();
    let rpcs: Vec<Client> = settings.rpcs.into_iter().map(|rpc| rpc.into()).collect();

    let mut results = vec![
        ["RPC", "Call", "Avg", "Best", "Worst"]
            .iter()
            .map(|s| s.to_string())
            .collect(),
    ];

    for rpc in rpcs {
        results.extend(rpc.test(settings.repeat.unwrap_or(3)).await);
    }

    let mut table = Builder::from(results).build();
    table.with(Style::modern_rounded());
    table.modify(Columns::new(2..5), Alignment::right());

    println!("{table}");
}
