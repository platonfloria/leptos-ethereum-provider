use leptos::*;

use crate::EthereumInterface;

#[component]
pub fn ConnectButton(
    cx: Scope,
    #[prop(optional)] children: Option<Children>,
    #[prop(optional)] connected_html: Option<HtmlElement<html::Div>>,
) -> impl IntoView {
    let ethereum = expect_context::<Option<EthereumInterface>>(cx);

    if let Some(ethereum) = ethereum {
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

        // children.
        let children = children.map(|f| f(cx));

        view! { cx,
            <div>
                {
                    move || {
                        let ethereum = ethereum.clone();
                        let connected_html = connected_html.clone();
                        let children = children.clone();
                        let connect = connect.clone();
                        let disconnect = disconnect.clone();
                        if ethereum.connected() {
                            view! { cx,
                                <div on:click=disconnect>
                                    {
                                        move || view! { cx,
                                            <div>
                                                {
                                                    let connected_html = connected_html.clone();
                                                    if let Some(connected_html) = connected_html {
                                                        view! { cx,
                                                            <div>
                                                                {connected_html}
                                                            </div>
                                                        }
                                                    } else {
                                                        view! { cx,
                                                            <div class="hover:shadow shadow btn connected">
                                                                <img src="./images/providers/metamask.svg" height="24" width="24" alt="metamask" class="inline-flex mr-2" />
                                                                {ethereum.display_short_address()}
                                                            </div>
                                                        }
                                                    }
                                                }
                                            </div>
                                        }
                                    }
                                </div>
                            }
                        } else {
                            view! { cx,
                                <div on:click=connect>
                                    {
                                        if let Some(children) = children {
                                            view! { cx,
                                                <div>
                                                    {children}
                                                </div>
                                            }
                                        } else {
                                            view! { cx,
                                                <div class="btn btn-primary disconnected">
                                                    "Connect Wallet"
                                                </div>
                                            }
                                        }
                                    }
                                </div>
                            }
                        }
                    }
                }
            </div>
        }
    } else {
        view! { cx, <div/> }
    }
}
