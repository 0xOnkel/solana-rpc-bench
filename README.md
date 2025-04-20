# Config
Provide a `config.toml` file which contains the RPCs that should be checked.

```toml
[[rpc]]
label = "Solana.com"
url = "https://api.mainnet-beta.solana.com"

[[rpc]]
label = "Helius"
url = "https://mainnet.helius-rpc.com/?api-key="
```

## Change # of repeats
By default, the test is run 3 times for each call. This can be changed with
setting the `repeat` variable in the `config.toml` file.

```toml
repeat = 5
```

# Usage
Just run the script with `cargo run --release` and you will get an output like
```
╭────────────┬───────────────────────┬────────┬────────┬────────╮
│ RPC        │ Call                  │    Avg │   Best │  Worst │
├────────────┼───────────────────────┼────────┼────────┼────────┤
│ Solana.com │ get_slot              │ 103 ms │ 103 ms │ 103 ms │
├────────────┼───────────────────────┼────────┼────────┼────────┤
│ Solana.com │ get_multiple_accounts │  74 ms │  74 ms │  74 ms │
├────────────┼───────────────────────┼────────┼────────┼────────┤
│ Helius     │ get_slot              │ 124 ms │ 124 ms │ 124 ms │
├────────────┼───────────────────────┼────────┼────────┼────────┤
│ Helius     │ get_multiple_accounts │ 105 ms │ 105 ms │ 105 ms │
├────────────┼───────────────────────┼────────┼────────┼────────┤
│ Quicknode  │ get_slot              │ 111 ms │ 111 ms │ 111 ms │
├────────────┼───────────────────────┼────────┼────────┼────────┤
│ Quicknode  │ get_multiple_accounts │  77 ms │  77 ms │  77 ms │
╰────────────┴───────────────────────┴────────┴────────┴────────╯
```
