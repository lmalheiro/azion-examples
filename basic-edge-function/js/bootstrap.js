import init, * as WasmModule from '../pkg/azion_rust_edge_function';
import wasmData from '../pkg/azion_rust_edge_function_bg.wasm';

let wasmPromise = null;

addEventListener("fetch", (event) => {

    try {

        if (!wasmPromise) {
            wasmPromise = fetch(wasmData).then(response => init(response.arrayBuffer()));
        }

        let resp = wasmPromise.then(() => WasmModule.fetch_listener(event)) ;
    
        event.respondWith(resp);
    
    } catch (e) {
        console.log(e.message);
        console.log(e.stack);
    }

});
