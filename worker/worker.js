addEventListener('fetch', event => {
  event.respondWith(handleRequest(event.request))
})

/**
 * Fetch and log a request
 * @param {Request} request
 */
async function handleRequest(request) {
    const { wasm_entry } = wasm_bindgen;
    await wasm_bindgen(wasm)
    return await wasm_entry(request);
}
