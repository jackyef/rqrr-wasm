
extern crate wasm_bindgen;
extern crate serde_json;
extern crate quirs;

use wasm_bindgen::prelude::*;
use quirs::Decoder;
use quirs::Image;
use quirs::Vec2D;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn decode_qr(js_array: JsValue, width: usize, height: usize) {
    let bytes: Vec<u8> = js_array.into_serde().unwrap();

    let mut decoder = Decoder::new().expect("can't create Qui-RS decoder");
    let image = Image::new(
        &bytes,
        Vec2D { x: width, y: height }
    ).expect(
        "can't create Qui-RS image"
    );
    let iter = decoder.decode_image(&image).expect("can't decode image");

    for code in iter {
        let code = code.expect("can't detect code");
        let info = code.decode().expect("can't decode code");

        log(info.as_str().expect("code contents not ASCII"));
    }
}