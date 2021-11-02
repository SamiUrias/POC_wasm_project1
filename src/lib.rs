use base64::{decode,encode};
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::console::log_1 as log;
use image::load_from_memory;
use image::ImageOutputFormat::Png;


#[wasm_bindgen]
pub fn grayscale(encoded_file: &str) -> String {
    log(&"grayscalle called".into());
    let base64_to_vector  = decode(encoded_file).unwrap();
    log(&"Image decoded".into());
    log(&"Image decoded1".into());
    
    // Image from memory
    let mut image = load_from_memory(&base64_to_vector).unwrap();
    log(&"Image loaded".into());
    image = image.grayscale();
    log(&"Grayscale effect applied".into());

    let mut buffer = vec![];
    image.write_to(&mut buffer, Png).unwrap();
    log(&"New Image is written".into());
    let encoded_image = encode(&buffer);
    let data_url = format!("data:image/png;base64,{}", encoded_image);
    return data_url;
}