mod utils;
mod rectangle;
mod matrix;
mod effect_transform;
pub mod silhouette;
pub mod drawable;

use wasm_bindgen::prelude::*;

use std::collections::HashMap;
use std::convert::TryInto;

use matrix::Matrix;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

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

    pub fn set_drawable(
        &mut self,
        id: drawable::DrawableID,
        matrix: Option<Box<[f32]>>,
        silhouette: Option<silhouette::SilhouetteID>,
        effects: Option<effect_transform::JSEffectMap>,
        effect_bits: effect_transform::EffectBits
    ) {
        let d = self.drawables.entry(id).or_insert(drawable::Drawable {
            matrix: [0.0; 16],
            inverse_matrix: [0.0; 16],
            effects: effect_transform::Effects::default(),
            effect_bits: 0,
            silhouette: match silhouette {
                Some(s) => s,
                None => ID_NONE
            },
            id
        });

        if let Some(m) = matrix {
            d.matrix = (*m).try_into().expect("drawable's matrix contains 16 elements");
            d.inverse_matrix = d.matrix.inverse();
        }
        if let Some(s) = silhouette {
            d.silhouette = s;
        }
        if let Some(fx) = effects {
            d.effects.set_from_js(fx);
        }
        d.effect_bits = effect_bits;
    }

    pub fn remove_drawable(&mut self, id: drawable::DrawableID) {
        self.drawables.remove(&id);
    }

    pub fn set_silhouette(
        &mut self,
        id: silhouette::SilhouetteID,
        w: u32,
        h: u32,
        data: Box<[u8]>,
        nominal_width:
        f64, nominal_height: f64
    ) {
        let s = self.silhouettes.entry(id).or_insert(silhouette::Silhouette::new(id));
        s.set_data(w, h, data, matrix::Vec2(nominal_width as f32, nominal_height as f32));
    }

    pub fn remove_silhouette(&mut self, id: silhouette::SilhouetteID) {
        self.silhouettes.remove(&id);
    }

    pub fn is_touching_drawables(
        &mut self,
        drawable: drawable::DrawableID,
        candidates:
        Vec<drawable::DrawableID>,
        rect: rectangle::JSRectangle
    ) -> bool {
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
                let position = matrix::Vec2(x as f32, y as f32);
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
