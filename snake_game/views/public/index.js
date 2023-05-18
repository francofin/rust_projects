/*
 * ATTENTION: The "eval" devtool has been used (maybe by default in mode: "development").
 * This devtool is neither made for production nor for readable output files.
 * It uses "eval()" calls to create a separate source file in the browser devtools.
 * If you are trying to read the output file, select a different devtool (https://webpack.js.org/configuration/devtool/)
 * or disable the default devtool with "devtool: false".
 * If you are looking for production-ready output files, see mode: "production" (https://webpack.js.org/configuration/mode/).
 */
/******/ (() => { // webpackBootstrap
/******/ 	var __webpack_modules__ = ({

/***/ "./index.js":
/*!******************!*\
  !*** ./index.js ***!
  \******************/
/***/ (() => {

eval("\r\n\r\nasync function init() {\r\n    alert(\"Hello There\");\r\n    // const byteArray = new Int8Array([0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00, 0x01, 0x07, 0x01, \r\n    // 0x60, 0x02, 0x7f, 0x7f, 0x01, 0x7f, 0x03, 0x02, 0x01, 0x00, 0x07, 0x07, 0x01, 0x03, \r\n    // 0x73, 0x75, 0x6d, 0x00, 0x00, 0x0a, 0x09, 0x01, 0x07, 0x00, 0x20, 0x00, 0x20, 0x01, 0x6a, \r\n    // 0x0b, 0x00, 0x18, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x01, 0x06, 0x01, 0x00, 0x03, 0x73, 0x75, 0x6d, 0x02, \r\n    // 0x09, 0x01, 0x00, 0x02, 0x00, 0x01, 0x61, 0x01, 0x01, 0x62]);\r\n    // const wasm  = await WebAssembly.instantiate(byteArray.buffer);\r\n    const res = await fetch(\"test.wasm\");\r\n    const buffer = await res.arrayBuffer();\r\n    // debugger;\r\n    const wasm  = await WebAssembly.instantiate(buffer);\r\n    const sumFunction = wasm.instance.exports.sum;\r\n    \r\n    // const result = sumFunction(50,1000);\r\n    console.log(result);\r\n\r\n\r\n}\r\n// alert(\"Hello There\");\r\ninit();\n\n//# sourceURL=webpack://views/./index.js?");

/***/ })

/******/ 	});
/************************************************************************/
/******/ 	
/******/ 	// startup
/******/ 	// Load entry module and return exports
/******/ 	// This entry module can't be inlined because the eval devtool is used.
/******/ 	var __webpack_exports__ = {};
/******/ 	__webpack_modules__["./index.js"]();
/******/ 	
/******/ })()
;