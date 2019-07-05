extern crate wasm_bindgen;

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
pub fn decode_qr(bytes: &[u8]) -> String {
    // let mut output = "".to_string();

    // for &var in bytes {
    //     output.push_str(&format!("{},", &var));
    // }

    // log(&output);

    let img = match image::load_from_memory(&bytes) {
        Ok(v) => v,
        Err(_e) => return format!("{}", "[Error] Failed when trying to load image"),
    };

    let img = img.to_luma();

    // Prepare for detection
    let mut img = rqrr::PreparedImage::prepare(img);
    // Search for grids, without decoding
    let grids = img.detect_grids();

    if grids.len() != 1 {
        return format!("{}", "[Error] No QR code detected in image")
    }

    // Decode the grid
    let (_meta, content) = match grids[0].decode() {
        Ok(v) => v,
        Err(_e) => return format!("{}", "[Error] Failed decoding the image"),
    };

    return format!("{}", content);
}
