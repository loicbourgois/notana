import init from "./wasm/wasm.js";


init().then( async (wasm) => {
    console.log('bob')
    // console.log(wasm.greet())
});
