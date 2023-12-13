

async function init() {

    const memory = new WebAssembly.Memory({initial: 1});

    const importObject = {
      js: {
        mem: memory,
      },
      console: {
        log: () => {
          console.log("Just logging something cool.");
        },
        error: () => {
          console.log("I am just an error");
        },
      }
    };

    const response = await fetch("sum.wasm");
    const buffer = await response.arrayBuffer();
    const wasm = await WebAssembly.instantiate(buffer, importObject);

    const sumFunction = wasm.instance.exports.sum;
    //const wasmMemory = wasm.instance.exports.mem;
    //const uint8 = new Uint8Array(wasmMemory.buffer, 0, 2);
    const uint8 = new Uint8Array(memory.buffer, 0, 2);
    const hiText = new TextDecoder().decode(uint8);
    console.log(hiText);
    debugger
    const result = sumFunction(200, 300);
    console.log(result);
  }
  
  init();