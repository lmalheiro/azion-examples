# Example on how to use Rust with Azion's Edge Functions

## The fetch listener

On file `src/lib.rs`, add your code to the fetch event listener, which will be always named `fetch_listener` and must be a public async function. 

```rust

#[wasm_bindgen]
pub async fn fetch_listener(event: &FetchEvent) -> Response {

    Response::new_with_opt_str(Some("Hello, World!")).unwrap()

}

```

## Building

Execute `npm run build`, then create and edge function in Azion using the output file `dist/worker.js`.

The example is configured to compile in debug mode and non-minified.