use leptos::*;
use leptos_ethereum_provider::{
    chain, AccountLabel, ConnectButton, EthereumContextProvider, SwitchNetworkButton,
};

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    view! { cx,
        <div>
            <EthereumContextProvider>
                <ConnectButton connected_html=view! { cx,
                    <div>
                        <button class="btn btn-primary connected">
                            "Disconnect"
                        </button>
                    </div>
                }>
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
    mount_to_body(|cx| view! { cx, <App/> })
}
