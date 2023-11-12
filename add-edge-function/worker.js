
/*
    WebAssembly and its equivalent binary data.
    
    (module
    (func (export "add") (param i32 i32) (result i32)
        local.get 0
        local.get 1
        i32.add
    )
    )
*/
const wasmBinary = new Uint8Array([
    0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00, 0x01, 0x07, 
    0x01, 0x60, 0x02, 0x7f, 0x7f, 0x01, 0x7f, 0x03, 0x02, 0x01, 
    0x00, 0x07, 0x07, 0x01, 0x03, 0x61, 0x64, 0x64, 0x00, 0x00, 
    0x0a, 0x09, 0x01, 0x07, 0x00, 0x20, 0x00, 0x20, 0x01, 0x6a, 
    0x0b
]);

// Instantiate WebAssembly module
const wasmModule = new WebAssembly.Module(wasmBinary);
const wasmInstance = new WebAssembly.Instance(wasmModule);

// Service Worker event listener
self.addEventListener('fetch', (event) => {
    event.respondWith(handleFetch(event));
});

// Handle fetch event
async function handleFetch(event) {

    // Extract numbers parameters
    const url = new URL(event.request.url);
    const numParam1 = parseInt(url.searchParams.get('num1'), 10) || 0;
    const numParam2 = parseInt(url.searchParams.get('num2'), 10) || 0;
  
    // Use the WebAssembly module to add numbers
    const result = wasmInstance.exports.add(numParam1, numParam2);

    // reutn a response with the result
    return new Response(`<h1>${result.toString()}</h1>\n`, {
        headers: { 'Content-Type': 'text/plain' }
    });
}


