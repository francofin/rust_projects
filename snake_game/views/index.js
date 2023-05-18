import init, { greet } from "snake_game"


// async function start {
//     const wasm = await init();
//     greet("Francois");
// }


init().then((wasm) => {
    console.log(wasm);
    greet("Francois");
    console.dir(init)
})


// async function init() {
//     alert("Hello There");
//     // const byteArray = new Int8Array([0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00, 0x01, 0x07, 0x01, 
//     // 0x60, 0x02, 0x7f, 0x7f, 0x01, 0x7f, 0x03, 0x02, 0x01, 0x00, 0x07, 0x07, 0x01, 0x03, 
//     // 0x73, 0x75, 0x6d, 0x00, 0x00, 0x0a, 0x09, 0x01, 0x07, 0x00, 0x20, 0x00, 0x20, 0x01, 0x6a, 
//     // 0x0b, 0x00, 0x18, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x01, 0x06, 0x01, 0x00, 0x03, 0x73, 0x75, 0x6d, 0x02, 
//     // 0x09, 0x01, 0x00, 0x02, 0x00, 0x01, 0x61, 0x01, 0x01, 0x62]);
//     // const wasm  = await WebAssembly.instantiate(byteArray.buffer);

//     //This creates one page of memory in ebassembly. We need to register the WebAssembly created from JS


//     const javascriptMemory = new WebAssembly.Memory({initial: 1})

//     const myImportedObject = {
//         js: {
//             jsMem: javascriptMemory,
//         },
//         console:{
//             log: () => {
//                 console.log("Logging Data");
//             },
//             error: () => {
//                 console.log("Error Recorded")
//             }
//         }
//     }
//     const res = await fetch("sum.wasm");
//     const buffer = await res.arrayBuffer();
//     // debugger;
//     const wasm  = await WebAssembly.instantiate(buffer, myImportedObject);
//     const sumFunction = wasm.instance.exports.sum;
//     const webAssemblyMemory = wasm.instance.exports.mem;
//     // const uint8Array = new Uint8Array(webAssemblyMemory.buffer, 0, 5);
    

//     const uint8Array = new Uint8Array(javascriptMemory.buffer, 0, 5);
//     const helloText = new TextDecoder().decode(uint8Array);
//     const result = sumFunction(50,1000);
//     console.log(result);
//     console.log(wasm);
//     // console.log(webAssemblyMemory);
//     console.log(helloText);

// }
// // alert("Hello There");
// init();