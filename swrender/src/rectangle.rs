use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    pub type JSRectangle;

    #[wasm_bindgen(method, getter)]
    pub fn left(this: &JSRectangle) -> f64;
    #[wasm_bindgen(method, getter)]
    pub fn right(this: &JSRectangle) -> f64;
    #[wasm_bindgen(method, getter)]
    pub fn bottom(this: &JSRectangle) -> f64;
    #[wasm_bindgen(method, getter)]
    pub fn top(this: &JSRectangle) -> f64;
}

pub struct Rectangle<T> {
    left: T,
    right: T,
    bottom: T,
    top: T
}

impl Rectangle<i32> {
    pub fn fromJSRectangle(rect: JSRectangle) -> Self {
        Rectangle {
            left: rect.left().floor() as i32,
            right: rect.right().ceil() as i32,
            bottom: rect.bottom().floor() as i32,
            top: rect.top().ceil() as i32
        }
    }
}
