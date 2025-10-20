use dioxus::prelude::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");

fn main() {
    dioxus::launch(App);
}

#[cfg(feature = "server")]
use dioxus_fullstack::FullstackContext;

#[cfg(feature = "server")]
use axum_macros::FromRequest;

/// A simple per-request context type
#[cfg(feature = "server")]
#[derive(FromRequest, Clone, Debug)]
struct AppContext {
    user_id: String,
}

#[server]
async fn trigger_bug() -> Result<String, ServerFnError> {
    // Await the context extraction
    let ctx: AppContext = FullstackContext::extract().await?;
    // tokio::spawn(async {
    // let ctx: AppContext = FullstackContext::extract().await.unwrap();
    // });

    Ok(format!("User: {}", ctx.user_id))
    // Ok("test".to_string())
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Hero {}
        Echo {}
    }
}

#[component]
pub fn Hero() -> Element {
    rsx! {
        div {
            id: "hero",
            img { src: HEADER_SVG, id: "header" }
            div { id: "links",
                a { href: "https://dioxuslabs.com/learn/0.6/", "📚 Learn Dioxus" }
                a { href: "https://dioxuslabs.com/awesome", "🚀 Awesome Dioxus" }
                a { href: "https://github.com/dioxus-community/", "📡 Community Libraries" }
                a { href: "https://github.com/DioxusLabs/sdk", "⚙️ Dioxus Development Kit" }
                a { href: "https://marketplace.visualstudio.com/items?itemName=DioxusLabs.dioxus", "💫 VSCode Extension" }
                a { href: "https://discord.gg/XgGxMSkvUM", "👋 Community Discord" }
            }
        }
        button {
            onclick: move |_| {
                spawn(async move {
                    match trigger_bug().await {
                        Ok(msg) => error!("Server fn OK: {msg}"),
                        Err(err) => error!("Server failed: {err}")
                    }
                });
            },
            "Trigger Context Bug"
        }
    }
}

/// Echo component that demonstrates fullstack server functions.
#[component]
fn Echo() -> Element {
    let mut response = use_signal(String::new);

    rsx! {
        div {
            id: "echo",
            h4 { "ServerFn Echo" }
            input {
                placeholder: "Type here to echo...",
                oninput:  move |event| async move {
                    let data = echo_server(event.value()).await.unwrap();
                    response.set(data);
                },
            }

            if !response().is_empty() {
                p {
                    "Server echoed: "
                    i { "{response}" }
                }
            }
        }
    }
}

/// Echo the user input on the server.
#[server(EchoServer)]
async fn echo_server(input: String) -> Result<String, ServerFnError> {
    Ok(input)
}
