use leptos::*;
use serde_json::json;
use wasm_bindgen::JsValue;
use web3::{
    futures::StreamExt,
    transports::eip_1193::{Eip1193, Provider},
    types::{H160, U256},
    Transport,
};

use crate::{Chain, ERC20Asset};


#[derive(Debug)]
pub struct EthereumState {
    pub connected: bool,
    pub accounts: Option<Vec<H160>>,
    pub chain_id: Option<U256>,
}

#[derive(Clone)]
pub struct EthereumInterface {
    pub provider: Provider,
    pub state: RwSignal<EthereumState>,
}

impl EthereumInterface {
    pub async fn connect(&self) -> Result<(), String> {
        let web3 = web3::Web3::new(Eip1193::new(self.provider.clone()));

        if let Ok(addresses) = web3.eth().request_accounts().await {
            log::info!("request_accounts() {:?}", addresses);

            let (_, connected) = self.connected_slice();
            let (_, accounts) = self.accounts_slice();
            let (_, chain_id) = self.chain_id_slice();

            connected.set(true);
            accounts.set(Some(addresses));
            chain_id.set(web3.eth().chain_id().await.ok());

            {
                let this = self.clone();
                spawn_local(async move {
                    this.on_chain_changed(|chain_id_| {
                        log::info!("event: chainChanged {:?}", chain_id);
                        chain_id.set(Some(
                            U256::from_dec_str(&chain_id_)
                                .expect(&format!("chain_id should be a valid U256 {}", &chain_id_)),
                        ));
                    })
                    .await;
                });
            }

            {
                let this = self.clone();
                spawn_local(async move {
                    log::info!("event: accountsChanged before");
                    this.on_accounts_changed(|addresses| {
                        log::info!("event: accountsChanged");
                        if addresses.is_empty() {
                            connected.set(false);
                        }
                        accounts.set(Some(addresses));
                    })
                    .await;
                });
            }

            {
                let this = self.clone();
                spawn_local(async move {
                    this.on_connect(|connect| {
                        log::info!("event: connect: {:?}", connect);
                        connected.set(true);
                    })
                    .await;
                });
            }

            {
                let this = self.clone();
                spawn_local(async move {
                    this.on_disconnect(|chain_id| {
                        log::info!("event: disconnect: {}", chain_id);
                        connected.set(false);
                    })
                    .await;
                });
            }
        };
        Ok(())
    }

    fn connected_slice(&self) -> (Signal<bool>, SignalSetter<bool>) {
        create_slice(
            self.state,
            |state| state.connected,
            |state, n| state.connected = n,
        )
    }

    fn accounts_slice(&self) -> (Signal<Option<Vec<H160>>>, SignalSetter<Option<Vec<H160>>>) {
        create_slice(
            self.state,
            |state| state.accounts.clone(),
            |state, n| state.accounts = n,
        )
    }

    fn chain_id_slice(&self) -> (Signal<Option<U256>>, SignalSetter<Option<U256>>) {
        create_slice(
            self.state,
            |state| state.chain_id,
            |state, n| state.chain_id = n,
        )
    }

    pub fn disconnect(&self) {
        let (_, connected) = self.connected_slice();
        connected.set(false);
    }

    pub fn connected(&self) -> bool {
        let (connected, _) = self.connected_slice();
        connected.get()
    }

    pub fn address(&self) -> Signal<Option<H160>> {
        let (accounts, _) = self.accounts_slice();
        Signal::derive(move || accounts.get().as_ref().and_then(|a| a.first()).copied())
    }

    /// returns the chain_id as a decimal. returns None on invalid chain values
    pub fn chain_id(&self) -> Signal<Option<u64>> {
        let (chain, _) = self.chain_id_slice();
        Signal::derive(move || chain.get().as_ref().map(U256::as_u64))
    }

    pub fn chain_id_hex(&self) -> Signal<Option<String>> {
        let (chain, _) = self.chain_id_slice();
        Signal::derive(move || chain
            .get()
            .as_ref()
            .map(|chain_id| format!("0x{:X}", chain_id)))
    }

    pub fn display_short_address(&self) -> Signal<String> {
        let address = self.address().clone();
        Signal::derive(move || address
            .get()
            .map(|address| address.to_string())
            // .map(|address| format!("0x{}", &address.split_at(2).1))
            .unwrap_or_default())
    }

    pub fn display_address(&self) -> Signal<String> {
        let address = self.address().clone();
        Signal::derive(move || address
            .get()
            .map(|add| format!("{:?}", add))
            .unwrap_or(String::new()))
    }

    pub async fn on_accounts_changed<F>(&self, callback: F)
    where
        F: Fn(Vec<web3::types::H160>),
    {
        let transport = Eip1193::new(self.provider.clone());
        let mut stream = transport.accounts_changed_stream();
        while let Some(accounts) = stream.next().await {
            log::info!("accounts changed");
            callback(accounts.clone());
        }
    }

    pub async fn on_chain_changed<F>(&self, callback: F)
    where
        F: Fn(String),
    {
        let transport = Eip1193::new(self.provider.clone());
        let mut stream = transport.chain_changed_stream();
        while let Some(chainid) = stream.next().await {
            callback(chainid.to_string());
        }
    }

    pub async fn on_connect<F>(&self, callback: F)
    where
        F: Fn(Option<String>),
    {
        let transport = Eip1193::new(self.provider.clone());
        let mut stream = transport.connect_stream();
        while let Some(connect) = stream.next().await {
            log::info!("QWEASD");
            callback(connect);
        }
    }

    pub async fn on_disconnect<F>(&self, callback: F)
    where
        F: Fn(String),
    {
        let transport = Eip1193::new(self.provider.clone());
        let mut stream = transport.disconnect_stream();
        while let Some(err) = stream.next().await {
            callback(err.to_string());
        }
    }

    /// switch chain or prompt user to add chain
    ///
    /// # Arguments
    /// * `chain` - a `Chain` instance representing the target chain
    ///
    pub async fn switch_chain_with_fallback(&self, chain: &Chain) -> Result<(), JsValue> {
        self.add_chain(chain).await?;
        self.switch_chain(&chain.chain_id).await?;
        Ok(())
    }

    /**
     * EIP-3326: Switch a wallet to another chain
     * https://eips.ethereum.org/EIPS/eip-3326
     * https://docs.metamask.io/guide/rpc-api.html#other-rpc-methods
     *
     * @param {number} chainId network chain identifier
     */
    pub async fn switch_chain(&self, chain_id: &str) -> Result<JsValue, JsValue> {
        log::info!("switch_chain");

        self.request(
            "wallet_switchEthereumChain",
            vec![json!({"chainId": chain_id})],
        )
        .await
        .map(|_| JsValue::from(chain_id))
        .map_err(|_| JsValue::from("error deserializing request params"))
    }

    /// EIP-3085: Add a wallet to another chain
    /// - https://eips.ethereum.org/EIPS/eip-3085
    /// - https://docs.metamask.io/guide/rpc-api.html#wallet-addethereumchain
    pub async fn add_chain(&self, chain: &Chain) -> Result<(), JsValue> {
        log::info!("add_chain");

        self.request("wallet_addEthereumChain", vec![json!(&chain)])
            .await
            .map(|_| ())
            .map_err(|_| JsValue::from("error deserializing request params"))
    }

    pub async fn watch_asset(&self, asset: &ERC20Asset) -> Result<(), JsValue> {
        log::info!("watch_asset");

        self.request(
            "wallet_watchAsset",
            vec![json!({
                "r#type": String::from("ERC20"),
                "options": asset
            })],
        )
        .await
        .map(|_| ())
        .map_err(|_| JsValue::from("error deserializing request params"))
    }

    pub async fn request(
        &self,
        method: &str,
        params: Vec<serde_json::Value>,
    ) -> web3::error::Result<serde_json::value::Value> {
        let transport = Eip1193::new(self.provider.clone());
        let (request_id, request) = transport.prepare(method, params);
        transport.send(request_id, request).await
    }
}
