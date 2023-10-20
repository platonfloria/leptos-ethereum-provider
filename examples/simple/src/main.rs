use leptos::*;
use leptos_ethereum_provider::{
    chain, AccountLabel, ConnectButton, EthereumContextProvider, SwitchNetworkButton,
};

#[component]
pub fn App() -> impl IntoView {
    view! {
        <div>
            <EthereumContextProvider>
                <ConnectButton connected_html=view! {
                    <button class="btn btn-primary connected">
                        "Disconnect"
                    </button>
                }.into_view()>
                    <button class="btn btn-primary disconnected">
                        "Connect"
                    </button>
                </ConnectButton>
                <SwitchNetworkButton chain=chain::ethereum()/>
                <SwitchNetworkButton chain=chain::avalanche_testnet()/>
                <AccountLabel/>
            </EthereumContextProvider>
        </div>
    }
}

fn main() {
    console_log::init_with_level(log::Level::Info).expect("could not initialize logger");
    mount_to_body(|| view! { <App/> })
}
