use base64::prelude::*;
use js_sys;
use js_sys::JsString;
use lazy_static::lazy_static;
use wasm_bindgen::prelude::*;
use web_sys::{console, FetchEvent, Response};
use yew::Properties;
use yew::prelude::*;
use yew::LocalServerRenderer;
use std::cmp::PartialEq;


// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

lazy_static! {
    static ref ICON: String = BASE64_STANDARD.encode(include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/rust-icon.png"
    )));
}

#[derive(Properties, Clone, PartialEq)]
struct AppProperties {
    name: String,
    now: JsString,
}

#[function_component]
fn App(prop: &AppProperties) -> Html {
    
    html! {
        <html>
            <head>
                <title>{ "Hello, hello!" }</title>
                <link rel={"icon"} href={ format!("data:;base64,{}", ICON.to_string()) }/>
            </head>

            <body>
              <h1>{ "Hello, world!!! How are you, "} {&prop.name} {"?"}</h1>
              <h2>
                { "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Cras porta suscipit eleifend. Aenean eu elit vitae leo
                lacinia malesuada eget interdum neque. Proin nec maximus ante. Nam malesuada convallis est, eu interdum erat. 
                Interdum et malesuada fames ac ante ipsum primis in faucibus. Donec feugiat lectus et enim efficitur fermentum. 
                Nunc pellentesque tortor nec lacinia lobortis. Fusce et risus vitae magna scelerisque varius. Aliquam erat lectus, 
                mattis et mauris eget, rutrum consequat nulla. Duis scelerisque quam nec lorem iaculis posuere. Praesent ultrices finibus 
                diam, eget commodo ipsum ornare eu. Ut at lectus semper purus pharetra porta. Morbi et leo sed turpis dignissim condimentum. 
                Nunc tincidunt arcu nibh." }
              </h2>
              <h3>
               { "The time now is: " }{ &prop.now }
              </h3>
            </body>
        </html>
    }
}

#[wasm_bindgen(js_name = fetch_listener)]
pub async fn listener(event: &FetchEvent) -> Response {
    console::log_1(&JsValue::from_str("Fetch Listener..."));

    let now = js_sys::Date::new_0().to_json();

    console::log_1(&now);

    let john_smith = String::from("John Smith");

    let name = event
        .request()
        .headers()
        .get("X-Name")
        .ok()
        .flatten()
        .unwrap_or(john_smith);

    let renderer = LocalServerRenderer::<App>::with_props(AppProperties {
        name,
        now,
    });

    let body = renderer.render().await;

    let response = Response::new_with_opt_str(Some(body.as_str())).unwrap();

    response.headers().set("Content-Type", "text/html").unwrap();

    response
}
