extern crate quirs;
extern crate lodepng;

use quirs::*;
use std::env::args;

fn main() {
    let filename = args().nth(1).expect("please specify a PNG filename");
    let image = lodepng::decode_file(
        &filename,
        lodepng::ColorType::GREY,
        8,
    ).expect(
        "error decoding PNG file"
    );
    let bitmap = match image {
        lodepng::Image::Grey(buf) => buf,
        _ => panic!("PNG couldn't be decoded as 8-bit grayscale"),
    };
    let bytes: Vec<u8> = bitmap.buffer.into_iter().map(|px| px.0).collect();

    let mut decoder = Decoder::new().expect("can't create Qui-RS decoder");
    let image = Image::new(
        &bytes,
        Vec2D { x: bitmap.width, y: bitmap.height }
    ).expect(
        "can't create Qui-RS image"
    );
    let iter = decoder.decode_image(&image).expect("can't decode image");

    for code in iter {
        let code = code.expect("can't detect code");
        let info = code.decode().expect("can't decode code");

        println!("{}", info.as_str().expect("code contents not ASCII"));
    }
}