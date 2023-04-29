//! velas-address-rust
//!
//! Rust lib for en/decoding address to velas/ether format
//!
//! ```rust
//! use velas_address_rust::*;
//!
//! let eth_addresses = "0x32Be343B94f860124dC4fEe278FDCBD38C102D88";
//! let vlx_addr = eth_to_vlx(eth_addresses).unwrap(); // V5dJeCa7bmkqmZF53TqjRbnB4fG6hxuu4f
//! let eth_addr = vlx_to_eth(&vlx_addr).unwrap(); // 0x32be343b94f860124dc4fee278fdcbd38c102d88
//! ```
//!
use basex_rs::{BaseX, Decode, Encode, BITCOIN};
use bitcoin_hashes::sha256;
use bitcoin_hashes::Hash;
use hex;
use regex::Regex;
use std::str;

fn hash_sha256(byte: &[u8]) -> String {
    format!("{}", sha256::Hash::hash(byte))
}

/// Convert ETH address to VLX address
///
/// ```rust
/// use velas_address_rust::*;
///
/// let eth_addresses = "0x32Be343B94f860124dC4fEe278FDCBD38C102D88";
/// assert_eq!(eth_to_vlx(eth_addresses).unwrap(), "V5dJeCa7bmkqmZF53TqjRbnB4fG6hxuu4f".to_string())
/// ```
///
pub fn eth_to_vlx(address: &str) -> Result<String, &str> {
    if address.is_empty() {
        return Err("Invalid address");
    }

    if !address.starts_with("0x") {
        return Err("Invalid address");
    }

    let clear_addr = match address.get(2..address.len()) {
        Some(addr) => addr.to_lowercase(),
        None => return Err("Invalid address"),
    };

    let hash_big = hash_sha256(hash_sha256(clear_addr.as_bytes()).as_bytes());
    let checksum = match hash_big.get(0..8) {
        Some(hash) => hash,
        None => return Err("Invalid address"),
    };

    let long_address = format!("{}{}", clear_addr, checksum);

    let bytes = hex::decode(long_address).unwrap().to_vec();

    let mut encode = BaseX::new(BITCOIN).encode(&bytes);

    if encode.len() < 33 {
        encode = format!("{}{}", "1".repeat(33 - encode.len()), encode);
    }

    Ok(format!("V{}", encode))
}

/// Convert VLX address to ETH address
///
/// ```rust
/// use velas_address_rust::*;
///
/// let vlx_addresses = "V5dJeCa7bmkqmZF53TqjRbnB4fG6hxuu4f";
/// assert_eq!(vlx_to_eth(vlx_addresses).unwrap(), "0x32be343b94f860124dc4fee278fdcbd38c102d88".to_string())
/// ```
///
pub fn vlx_to_eth(address: &str) -> Result<String, &str> {
    if address.is_empty() {
        return Err("Invalid address");
    }

    if !address.starts_with("V") {
        return Err("Invalid address");
    }

    let clear_addr = match address.get(1..address.len()) {
        Some(addr) => addr,
        None => return Err("Invalid address"),
    };

    let decode_addr = match BaseX::new(BITCOIN).decode(clear_addr.to_string()) {
        Some(bytes) => bytes,
        None => return Err("Invalid address"),
    };

    let hex = hex::encode(decode_addr);

    let re = Regex::new(r"([0-9abcdef]+)([0-9abcdef]{8})").unwrap();

    let caps = re.captures(&hex).unwrap();

    if caps.len() != 3 as usize {
        return Err("Invalid address");
    }

    let mut match_addr = &caps[1];

    if match_addr.len() > 40 {
        let len = match_addr.len() - 40;
        if match_addr.starts_with(&"0".repeat(len)) {
            match_addr = match match_addr.get(len..match_addr.len()) {
                Some(addr) => addr,
                None => return Err("Invalid address"),
            }
        } else {
            return Err("Invalid address");
        }
    }

    let hash_big = hash_sha256(hash_sha256(match_addr.as_bytes()).as_bytes());
    let checksum = match hash_big.get(0..8) {
        Some(hash) => hash,
        None => return Err("Invalid address"),
    };

    if checksum != &caps[2] {
        return Err("Invalid checksum");
    }

    Ok(format!("0x{}", match_addr))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let eth_addresses = [
            "0x32Be343B94f860124dC4fEe278FDCBD38C102D88",
            "0x000000000000000000000000000000000000000f",
            "0xf000000000000000000000000000000000000000",
            "0x0000000000000000000000000000000000000001",
            "0x1000000000000000000000000000000000000000",
            "0x0000000000000000000000000000000000000000",
            "0xffffffffffffffffffffffffffffffffffffffff",
        ];

        let vlx_addresses = [
            "V5dJeCa7bmkqmZF53TqjRbnB4fG6hxuu4f",
            "V111111111111111111111111112jSS6vy",
            "VNt1B3HD3MghPihCxhwMxNKRerBPPbiwvZ",
            "V111111111111111111111111111CdXjnE",
            "V2Tbp525fpnBRiSt4iPxXkxMyf5ZX7bGAJ",
            "V1111111111111111111111111113iMDfC",
            "VQLbz7JHiBTspS962RLKV8GndWFwdcRndD",
        ];

        for addr in eth_addresses.iter() {
            let vlx_addr = eth_to_vlx(addr).unwrap();
            let eth_addr = vlx_to_eth(&vlx_addr).unwrap();
            assert_eq!(eth_addr.to_string(), addr.to_string().to_lowercase());
        }

        for addr in vlx_addresses.iter() {
            let eth_addr = vlx_to_eth(addr).unwrap();
            let vlx_addr = eth_to_vlx(&eth_addr).unwrap();
            assert_eq!(vlx_addr.to_string(), addr.to_string());
        }
    }
}
