use leptos::prelude::*;
use web3::transports::eip_1193::Provider;

use crate::{EthereumInner, EthereumState};

#[component]
pub fn EthereumContextProvider(children: Children) -> impl IntoView {
    let interface = if let Ok(Some(provider)) = Provider::default() {
        Some(LocalStorage::wrap(EthereumInner {
            provider,
            state: RwSignal::new(EthereumState {
                connected: false,
                accounts: None,
                chain_id: None,
            }),
        }))
    } else {
        None
    };
    provide_context(interface);

    children()
}
