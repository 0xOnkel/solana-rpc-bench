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
    let count = settings.repeat.unwrap_or(3);
    let rpcs: Vec<Client> = settings.rpcs.into_iter().map(|rpc| rpc.into()).collect();

    let mut results = vec![vec!["".to_string()]];

    for (i, rpc) in rpcs.iter().enumerate() {
        results[0].push(rpc.label.clone());
        if i > 0 {
            for (j, result) in rpc.test(count).await.iter().enumerate() {
                results[j + 1].push(result[1].clone());
            }
        } else {
            results.extend(rpc.test(count).await);
        }
    }

    let stats = [
        vec!["".to_string()],
        (0..rpcs.len())
            .map(|_| "   avg |   best |  worst".to_string())
            .collect(),
    ]
    .concat();

    results.push(stats);

    let mut table = Builder::from(results).build();
    table.with(Style::modern_rounded());
    // table.modify(Columns::new(2..5), Alignment::right());

    println!("{table}");
}
