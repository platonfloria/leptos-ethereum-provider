use leptos::*;

use crate::EthereumInterface;


#[component]
pub fn AccountLabel() -> impl IntoView {
    let ethereum = expect_context::<Option<EthereumInterface>>();

    view! {
        <div>
            {
                move || {
                    let ethereum = ethereum.clone();
                    if let Some(ethereum) = ethereum {
                        if ethereum.connected() {
                            ethereum.display_address().get()
                        } else {
                            "Disconnected".into()
                        }
                    } else {
                        "No ethereum provider found".into()
                    }
                }
            }
        </div>
    }
}
