mod utils;
mod matrix;
pub mod silhouette;
pub mod drawable;

use wasm_bindgen::prelude::*;

use std::collections::HashMap;
use std::convert::TryInto;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);

    pub type Rectangle;

    #[wasm_bindgen(method, getter)]
    fn left(this: &Rectangle) -> f64;
    #[wasm_bindgen(method, getter)]
    fn right(this: &Rectangle) -> f64;
    #[wasm_bindgen(method, getter)]
    fn bottom(this: &Rectangle) -> f64;
    #[wasm_bindgen(method, getter)]
    fn top(this: &Rectangle) -> f64;
}

const ID_NONE: u32 = u32::max_value();

#[wasm_bindgen]
pub struct SoftwareRenderer {
    drawables: HashMap<drawable::DrawableID, drawable::Drawable>,
    silhouettes: HashMap<silhouette::SilhouetteID, silhouette::Silhouette>
}

#[wasm_bindgen]
impl SoftwareRenderer {
    pub fn new() -> SoftwareRenderer {
        let mut renderer = SoftwareRenderer {
            drawables: HashMap::new(),
            silhouettes: HashMap::new()
        };

        renderer.silhouettes.insert(ID_NONE, silhouette::Silhouette::new(ID_NONE));

        utils::set_panic_hook();
        renderer
    }

    pub fn set_drawable(&mut self, id: drawable::DrawableID, matrix: Box<[f32]>, inverse_matrix: Box<[f32]>, silhouette: Option<silhouette::SilhouetteID>) {
        let d = self.drawables.entry(id).or_insert(drawable::Drawable {
            matrix: [0.0; 16],
            inverse_matrix: [0.0; 16],
            silhouette: match silhouette {
                Some(s) => s,
                None => ID_NONE
            },
            id
        });

        d.matrix = (*matrix).try_into().expect("drawable's matrix contains 16 elements");
        d.inverse_matrix = (*inverse_matrix).try_into().expect("drawable's inverse matrix contains 16 elements");
        if let Some(s) = silhouette {
            d.silhouette = s;
        }
    }

    pub fn remove_drawable(&mut self, id: drawable::DrawableID) {
        self.drawables.remove(&id);
    }

    pub fn set_silhouette(&mut self, id: silhouette::SilhouetteID, w: u32, h: u32, data: Box<[u8]>) {
        let s = self.silhouettes.entry(id).or_insert(silhouette::Silhouette::new(id));
        s.set_data(w, h, data);
    }

    pub fn remove_silhouette(&mut self, id: silhouette::SilhouetteID) {
        self.silhouettes.remove(&id);
    }

    pub fn is_touching_drawables(&mut self, drawable: drawable::DrawableID, candidates: Vec<drawable::DrawableID>, rect: Rectangle) -> bool {
        let left = rect.left() as i32;
        let right = rect.right() as i32 + 1;
        let bottom = rect.bottom() as i32 - 1;
        let top = rect.top() as i32;

        let drawable = self.drawables.get(&drawable).expect("Drawable should exist");
        let silhouette = self.silhouettes.get(&drawable.silhouette).unwrap();
        let candidates: Vec<(&drawable::Drawable, &silhouette::Silhouette)> = candidates.into_iter()
            .map(|c| {
                let d = self.drawables.get(&c).expect("Candidate drawable should exist");
                let s = self.silhouettes.get(&d.silhouette).unwrap();
                (d, s)
            }).collect();

        for x in left..right {
            for y in bottom..top {
                let position = (x as f32, y as f32);
                if drawable.is_touching(position, silhouette) {
                    for candidate in &candidates {
                        if candidate.0.is_touching(position, candidate.1) {
                            return true;
                        }
                    }
                }
            }
        }

        false
    }
}
