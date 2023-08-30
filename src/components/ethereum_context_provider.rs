use leptos::*;
use web3::transports::eip_1193::Provider;

use crate::{EthereumInterface, EthereumState};

#[component]
pub fn EthereumContextProvider(cx: Scope, children: Children) -> impl IntoView {
    let interface = if let Ok(Some(provider)) = Provider::default() {
        Some(EthereumInterface {
            cx: cx.clone(),
            provider,
            state: create_rw_signal(
                cx,
                EthereumState {
                    connected: false,
                    accounts: None,
                    chain_id: None,
                },
            ),
        })
    } else {
        None
    };
    provide_context(cx, interface);

    view! { cx,
        <div>
            {children(cx)}
        </div>
    }
}
