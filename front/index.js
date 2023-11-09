import init from "./wasm/wasm.js";

const md = `
# Bake a cake


`


init().then( async (wasm) => {
    console.log('bob')
    // console.log(wasm.greet())
});
