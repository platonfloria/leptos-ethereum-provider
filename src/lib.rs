//! Yew components for metamask and other eip1193 clients
pub mod base_currency;
pub mod chain;
mod components;
mod interface;

pub use components::*;
pub use interface::*;


/// A descriptor for an ethereum-compatible chain
#[derive(serde::Serialize, Default, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Chain {
    /// hex-based id of an ethereum compatible chain (eg. "0x01")
    pub chain_id: String,
    /// Name of the chain
    pub chain_name: String,
    /// Array of RPC endpoints as urls
    pub rpc_urls: [String; 1],
    /// Base currency of the chain
    pub native_currency: BaseCurrency,

    /// Block explorer urls
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_explorer_urls: Option<[String; 1]>,
}

/// Metadata for an ERC20 asset.
#[derive(serde::Serialize, Default, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ERC20Asset {
    /// public address of token contract
    pub address: String,
    /// ticker symbol used (eg USDC)
    pub token_symbol: String,
    /// decimal places (usually 8)
    pub decimals: u32,
    /// url for the token
    pub image_url: String,
}

/// A base currency for en ethereum compatible chain
#[derive(serde::Serialize, Default, PartialEq, Clone)]
pub struct BaseCurrency {
    /// currency name
    pub name: String,
    /// ticker symbol of the currency
    pub symbol: String, // 2-6 characters long
    /// number of decimal places (usually 8)
    pub decimals: u32,
}
