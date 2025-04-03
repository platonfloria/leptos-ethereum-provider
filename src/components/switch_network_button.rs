use crate::{Chain, EthereumInterface};
use leptos::{prelude::*, task::spawn_local};

#[component]
pub fn SwitchNetworkButton(chain: Chain, #[prop(optional)] class: Option<String>) -> impl IntoView {
    let ethereum = expect_context::<Option<EthereumInterface>>();

    ethereum.map(|ethereum| {
        let chain = chain.clone();

        let on_click = {
            let chain = chain.clone();
            move |_| {
                let ethereum = ethereum.clone();
                let chain = chain.clone();
                spawn_local(async move {
                    let _ = ethereum.switch_chain_with_fallback(&chain).await;
                });
            }
        };

        view! {
            <div>
                <button on:click=on_click class=class>
                    "Switch to "{chain.chain_name}
                </button>
            </div>
        }
    })
}
