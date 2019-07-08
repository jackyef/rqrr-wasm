import '../docs/qr_rust.js'; // include the js bindings
import wasmUrl from '../docs/qr_rust.wasm'; // we will use url loader for this file

const wasmModule = {
  init: () => {
    // a promise. If resolved, then the wasm module is ready to be used.
    return wasm_bindgen(wasmUrl);
  },
  decode: uint8Array => {
    const { decode_qr } = wasm_bindgen;

    try {
      const output = decode_qr(uint8Array);

      return output;
    } catch (err) {
      console.err("[Error while decoding]", err);

      return err;
    }
  }
}

export default wasmModule;
