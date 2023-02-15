import init, * as WasmModule from '../pkg/azion_rust_edge_function';
import wasmData from '../pkg/azion_rust_edge_function_bg.wasm';


let wasmPromise = fetch(wasmData).then(response => init(response.arrayBuffer()));


addEventListener("fetch", async (event) => {

    try {

        await wasmPromise;

        let resp = await WasmModule.fetch_listener(event);
    
        event.respondWith(resp);
    
    } catch (e) {
        console.log(e.message);
        console.log(e.stack);
    }

});
