const wasmModule = require("./wasm/pkg_node/wasm.js");

async function run() {
    // Read all stdin into a Buffer
    const chunks = [];
    for await (const chunk of process.stdin) {
      chunks.push(chunk);
    }
    const buffer = Buffer.concat(chunks);

    // Convert Node.js Buffer to Uint8Array for WASM
    const bytes = new Uint8Array(buffer);
    const hash = wasmModule.process_image(bytes);
    console.log(hash);
}

run();

