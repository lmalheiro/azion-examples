# Example of a Vulcan preset for a Rust/Wasm Azion Edge Function

This example demonstrates how define a Rust/Wasm preset for `vulcan`.

## Preparing Vulcan

Install [`vulcan`](https://github.com/aziontech/vulcan)

Create the preset `rustwasm` for `compute` mode:

`vulcan presets create`

Copy the preset files from `./preset` to the directory indicated by the command above (this is necessary only because the preset are not yet available in the official repository).

## Building

Execute `vulcan build`.

