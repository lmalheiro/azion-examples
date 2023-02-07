use js_sys;
use wasm_bindgen::prelude::*;
use web_sys::{console, FetchEvent, Response};

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub async fn fetch_listener(event: &FetchEvent) -> Response {
    console::log_1(&JsValue::from_str("Fetch Listener..."));

    let now = &js_sys::Date::new_0().to_json();

    console::log_1(now);

    let john_smith = String::from("John Smith");

    let name = event
        .request()
        .headers()
        .get("X-Name")
        .ok()
        .flatten()
        .unwrap_or(john_smith);

    let body = format!(
        r#"
        <!DOCTYPE html>
        <html>
            <head>
                <title>Hello, hello!</title>
            </head>
            <body>
            <h1>Hello, world!!! How are you, {name}?</h1>
              <h2>
                Lorem ipsum dolor sit amet, consectetur adipiscing elit. Cras porta suscipit eleifend. Aenean eu elit vitae leo 
                lacinia malesuada eget interdum neque. Proin nec maximus ante. Nam malesuada convallis est, eu interdum erat. 
                Interdum et malesuada fames ac ante ipsum primis in faucibus. Donec feugiat lectus et enim efficitur fermentum. 
                Nunc pellentesque tortor nec lacinia lobortis. Fusce et risus vitae magna scelerisque varius. Aliquam erat lectus, 
                mattis et mauris eget, rutrum consequat nulla. Duis scelerisque quam nec lorem iaculis posuere. Praesent ultrices finibus 
                diam, eget commodo ipsum ornare eu. Ut at lectus semper purus pharetra porta. Morbi et leo sed turpis dignissim condimentum. 
                Nunc tincidunt arcu nibh. 
              </h2>
              <h3>
               The time now is: {now}
              </h3>
            </body>
        </html>
    "#
    );

    let body = body.as_str();

    let response = Response::new_with_opt_str(Some(body)).unwrap();

    response.headers().set("Content-Type", "text/html").unwrap();
    response
}
