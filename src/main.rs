use solana_rpc_bench::setting::Settings;

fn main() {
    // this should help, but is not mandatory
    // user can provide the env vars in another way
    let _ = dotenvy::dotenv();

    let settings = Settings::new();
    println!("{settings:#?}");
}
