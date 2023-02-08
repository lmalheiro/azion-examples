# An example on how to use Rust with Azion's Edge Functions

## The fetch listener

On file `src/lib.rs`, add your code to the fetch event listener, which must have the JS name `fetch_listener` and must be a public async function.

```rust

#[wasm_bindgen]
pub async fn fetch_listener(event: &FetchEvent) -> Response {

    Response::new_with_opt_str(Some("Hello, World!")).unwrap()

}

```

You can use the attribute `js_name` to bind a rust function with a non matching name to `fetch_listener`:

```rust
#[wasm_bindgen(js_name = fetch_listener)]
pub async fn listener(event: &FetchEvent) -> Response {
...
}
```

## Building

Execute `npm run build`, then create and an edge function in Azion using the output file `dist/worker.js`.

The example is configured to compile in debug mode and isn't minified.