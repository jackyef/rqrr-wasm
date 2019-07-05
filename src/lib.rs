extern crate wasm_bindgen;
extern crate serde_json;

use wasm_bindgen::prelude::*;
use image;
use rqrr;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn decode_qr(js_array: JsValue) -> String {
    let bytes: Vec<u8> = js_array.into_serde().unwrap();

    let img = image::load_from_memory(&bytes).expect("Failed loading image from memory").to_luma();
    // Prepare for detection
    let mut img = rqrr::PreparedImage::prepare(img);
    // Search for grids, without decoding
    let grids = img.detect_grids();
    assert_eq!(grids.len(), 1);
    // Decode the grid
    let (_meta, content) = match grids[0].decode() {
        Ok(v) => v,
        Err(_e) => return format!("{}", "null"),
    };

    log(&content);

    format!("{}", content)
}