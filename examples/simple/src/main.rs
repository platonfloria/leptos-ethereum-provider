use leptos::prelude::*;
use leptos_ethereum_provider::{
    chain, AccountLabel, ConnectButton, EthereumContextProvider, EthereumInterface,
    SwitchNetworkButton,
};

#[component]
pub fn Contents() -> impl IntoView {
    let ethereum = expect_context::<Option<EthereumInterface>>();

    ethereum.map(|ethereum| {
        let disconnect = {
            let ethereum = ethereum.clone();
            move |_| ethereum.disconnect()
        };

        view! {
            <ConnectButton connected_html=view! {
                <button on:click=disconnect class="btn btn-primary connected">
                    "Disconnect"
                </button>
            }>
                <button class="btn btn-primary disconnected">
                    "Connect"
                </button>
            </ConnectButton>
            <SwitchNetworkButton chain=chain::ethereum()/>
            <SwitchNetworkButton chain=chain::avalanche_testnet()/>
            <AccountLabel/>
        }
    })
}

#[component]
pub fn App() -> impl IntoView {
    view! {
        <div>
            <EthereumContextProvider>
                <Contents/>
            </EthereumContextProvider>
        </div>
    }
}

fn main() {
    console_log::init_with_level(log::Level::Info).expect("could not initialize logger");
    mount_to_body(|| view! { <App/> })
}
