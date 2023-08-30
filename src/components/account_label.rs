use leptos::*;

use crate::EthereumInterface;

#[component]
pub fn AccountLabel(cx: Scope) -> impl IntoView {
    let ethereum = expect_context::<Option<EthereumInterface>>(cx);

    view! { cx,
        <div>
            {
                move || {
                    let ethereum = ethereum.clone();
                    if let Some(ethereum) = ethereum {
                        if ethereum.connected() {
                            ethereum.display_address()
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
