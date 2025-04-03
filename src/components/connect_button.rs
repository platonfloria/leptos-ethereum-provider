use leptos::{prelude::*, task::spawn_local};

use crate::EthereumInterface;

#[component]
pub fn ConnectButton(
    #[prop(optional)] children: Option<ChildrenFn>,
    #[prop(optional)] connected_html: Option<impl IntoView + Clone + 'static>,
) -> impl IntoView {
    let ethereum = expect_context::<Option<EthereumInterface>>();

    ethereum.map(|ethereum| {
        let connect = {
            let ethereum = ethereum.clone();
            move |_| {
                let ethereum = ethereum.clone();
                spawn_local(async move {
                    let _ = ethereum.connect().await;
                });
            }
        };

        let disconnect = {
            let ethereum = ethereum.clone();
            move |_| ethereum.disconnect()
        };

        let children = || children.map(|f| f());

        view! {
            <div>
                {
                    move || {
                        let ethereum = ethereum.clone();
                        let connected_html = connected_html.clone();
                        let children = children.clone();
                        let connect = connect.clone();
                        let disconnect = disconnect.clone();
                        if ethereum.connected() {
                            view! {
                                <div>
                                    {
                                        move || {
                                            let disconnect = disconnect.clone();
                                            view! {
                                                <div>
                                                    {
                                                        let connected_html = connected_html.clone();
                                                        if let Some(connected_html) = connected_html {
                                                            view! {
                                                                <div>
                                                                    {connected_html}
                                                                </div>
                                                            }.into_any()
                                                        } else {
                                                            view! {
                                                                <div on:click=disconnect class="hover:shadow shadow btn connected">
                                                                    <img src="./images/providers/metamask.svg" height="24" width="24" alt="metamask" class="inline-flex mr-2" />
                                                                    {ethereum.display_short_address()}
                                                                </div>
                                                            }.into_any()
                                                        }
                                                    }
                                                </div>
                                            }
                                        }
                                    }
                                </div>
                            }.into_any()
                        } else {
                            view! {
                                <div on:click=connect>
                                    {
                                        if let Some(children) = children() {
                                            view! {
                                                <div>
                                                    {children}
                                                </div>
                                            }.into_any()
                                        } else {
                                            view! {
                                                <div class="btn btn-primary disconnected">
                                                    "Connect Wallet"
                                                </div>
                                            }.into_any()
                                        }
                                    }
                                </div>
                            }.into_any()
                        }
                    }
                }
            </div>
        }
    })
}
