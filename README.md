# velas-address-rust

![crates.io](https://img.shields.io/crates/v/velas-address-rust.svg)
![docs.rs](https://docs.rs/velas-address-rust/badge.svg)

Rust lib for en/decoding address to velas/ether format

# Usage
```rust
use velas_address_rust::*;

fn main() {
    let eth_addresses = "0x32Be343B94f860124dC4fEe278FDCBD38C102D88";
    let vlx_addr = eth_to_vlx(eth_addresses).unwrap(); // V5dJeCa7bmkqmZF53TqjRbnB4fG6hxuu4f
    let eth_addr = vlx_to_eth(&vlx_addr).unwrap(); // 0x32be343b94f860124dc4fee278fdcbd38c102d88
}
```