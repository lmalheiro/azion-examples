# Example on how to use Rust with Azion's Edge Functions

## The fetch listener

On file `src/lib.rs` add code to fetch event listener. 

```rust

#[wasm_bindgen]
pub async fn fetch_listener(event: &FetchEvent) -> Response {

    Response::new_with_opt_str(Some("Hello, World!")).unwrap()

}

```

## Building

Execute `npm run build`, then create and edge function in Azion using the output file `dist/worker.js`.

The example is configured to compile in debug mode.