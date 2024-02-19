# Payment Splitter for SEI
Payment Splitter for SEI

Simple call with empty payload to disburse funds to defined wallets. Number of wallets can be hardcoded or passed in at initialization.

This is a bulk send and is simplified and does not include the other pull methods of payment claiming for individual wallets, due to the cheap nature of SEI fee costs.

**THIS IS PROVIDED WITHOUT ANY EXPECTED WARRANTY OR SUPPORT. USE AT YOUR OWN RISK.**


## Build

To build:

```cargo build```

```cargo wasm```

```
docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/rust-optimizer:0.14.0
```