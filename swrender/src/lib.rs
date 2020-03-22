mod utils;
mod matrix;
mod effect_transform;
mod convex_hull;
pub mod silhouette;
pub mod drawable;

use wasm_bindgen::prelude::*;

use std::collections::HashMap;
use std::convert::TryInto;

use matrix::Matrix;

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

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

const ID_NONE: drawable::DrawableID = -1;

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
        effect_bits: effect_transform::EffectBits,
        use_nearest_neighbor: bool
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
            use_nearest_neighbor,
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
        d.use_nearest_neighbor = use_nearest_neighbor;
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
        nominal_width: f64,
        nominal_height: f64,
        premultiplied: bool,
    ) {
        let s = self.silhouettes.entry(id).or_insert(silhouette::Silhouette::new(id));
        s.set_data(w, h, data, matrix::Vec2(nominal_width as f32, nominal_height as f32), premultiplied);
    }

    pub fn remove_silhouette(&mut self, id: silhouette::SilhouetteID) {
        self.silhouettes.remove(&id);
    }

    fn map_candidates(
        &self,
        candidates: Vec<drawable::DrawableID>
    ) -> Vec<(&drawable::Drawable, &silhouette::Silhouette)> {
        candidates.into_iter()
        .map(|c| {
            let d = self.drawables.get(&c).expect("Candidate drawable should exist");
            let s = self.silhouettes.get(&d.silhouette).unwrap();
            (d, s)
        }).collect()
    }

    fn per_rect_pixel<F>(
        &self,
        func: F,
        rect: JSRectangle,
        drawable: drawable::DrawableID
    ) -> bool
        where F: Fn(
            matrix::Vec2,
            &drawable::Drawable,
            &silhouette::Silhouette
        ) -> bool {

        let left = rect.left() as i32;
        let right = rect.right() as i32 + 1;
        let bottom = rect.bottom() as i32 - 1;
        let top = rect.top() as i32;

        let drawable = self.drawables.get(&drawable).expect("Drawable should exist");
        let silhouette = self.silhouettes.get(&drawable.silhouette).unwrap();

        for y in bottom..top {
            for x in left..right {
                let position = matrix::Vec2(x as f32, y as f32);
                if func(position, drawable, silhouette) {
                    return true;
                }
            }
        }

        false
    }

    pub fn is_touching_drawables(
        &mut self,
        drawable: drawable::DrawableID,
        candidates: Vec<drawable::DrawableID>,
        rect: JSRectangle
    ) -> bool {
        let candidates = self.map_candidates(candidates);
        self.per_rect_pixel(|
            position,
            drawable,
            silhouette
        | {
            if drawable.is_touching(position, silhouette) {
                for candidate in &candidates {
                    if candidate.0.is_touching(position, candidate.1) {
                        return true;
                    }
                }
            }
            false
        }, rect, drawable)
    }

    #[inline(always)]
    fn color_matches(
        a: [u8; 3],
        b: [u8; 3]
    ) -> bool {
        (
            ((a[0] ^ b[0]) & 0b11111000) |
            ((a[1] ^ b[1]) & 0b11111000) |
            ((a[2] ^ b[2]) & 0b11110000)
        ) == 0
    }

    #[inline(always)]
    fn mask_matches(
        a: [u8; 4],
        b: [u8; 3]
    ) -> bool {
        a[3] != 0 &&
        (
            ((a[0] ^ b[0]) & 0b11111100) |
            ((a[1] ^ b[1]) & 0b11111100) |
            ((a[2] ^ b[2]) & 0b11111100)
        ) == 0
    }

    pub fn color_is_touching_color(
        &mut self,
        drawable: drawable::DrawableID,
        candidates: Vec<drawable::DrawableID>,
        rect: JSRectangle,
        color: &[u8],
        mask: &[u8]
    ) -> bool {
        let color: [u8; 3] = (*color).try_into().expect("color contains 3 elements");
        let mask: [u8; 3] = (*mask).try_into().expect("mask contains 3 elements");
        let candidates = self.map_candidates(candidates);

        self.per_rect_pixel(|
            position,
            drawable,
            silhouette
        | {
            if Self::mask_matches(drawable.sample_color(position, silhouette), mask) {
                let sample_color = self.sample_color(position, &candidates);
                if Self::color_matches(color, sample_color) {
                    return true;
                }
            }
            false
        }, rect, drawable)
    }

    pub fn is_touching_color(
        &mut self,
        drawable: drawable::DrawableID,
        candidates: Vec<drawable::DrawableID>,
        rect: JSRectangle,
        color: &[u8]
    ) -> bool {
        let color: [u8; 3] = (*color).try_into().expect("color contains 3 elements");
        let candidates = self.map_candidates(candidates);
        self.per_rect_pixel(|
            position,
            drawable,
            silhouette
        | {
            if drawable.is_touching(position, silhouette) {
                let sample_color = self.sample_color(position, &candidates);
                if Self::color_matches(color, sample_color) {
                    return true;
                }
            }
            false
        }, rect, drawable)
    }

    fn sample_color(
        &self,
        position: matrix::Vec2,
        candidates: &Vec<(&drawable::Drawable, &silhouette::Silhouette)>
    ) -> [u8; 3] {
        let mut dst_color: (f32, f32, f32, f32) = (0f32, 0f32, 0f32, 0f32);
        let mut blend_alpha = 1f32;

        for candidate in candidates.into_iter() {
            let col = candidate.0.sample_color(position, candidate.1);
            dst_color.0 += (col[0] as f32) * blend_alpha;
            dst_color.1 += (col[1] as f32) * blend_alpha;
            dst_color.2 += (col[2] as f32) * blend_alpha;
            blend_alpha *= 1f32 - (col[3] as f32 / 255f32);

            if blend_alpha == 0f32 {
                break;
            }
        }

        let alpha8 = blend_alpha * 255f32;
        dst_color.0 += alpha8;
        dst_color.1 += alpha8;
        dst_color.2 += alpha8;

        [dst_color.0 as u8, dst_color.1 as u8, dst_color.2 as u8]
    }

    pub fn drawable_touching_rect(
        &mut self,
        drawable: drawable::DrawableID,
        rect: JSRectangle
    ) -> bool {
        self.per_rect_pixel(|
            position,
            drawable,
            silhouette
        | {
            if drawable.is_touching(position, silhouette) {
                return true;
            }
            false
        }, rect, drawable)
    }

    pub fn pick(
        &mut self,
        candidates: Vec<drawable::DrawableID>,
        rect: JSRectangle
    ) -> drawable::DrawableID {
        let mut hits: HashMap<drawable::DrawableID, u32> = HashMap::new();
        hits.insert(ID_NONE, 0);

        let candidates = self.map_candidates(candidates);

        // TODO: deduplicate with per_rect_pixel
        let left = rect.left() as i32;
        let right = rect.right() as i32 + 1;
        let bottom = rect.bottom() as i32 - 1;
        let top = rect.top() as i32;

        for y in bottom..top {
            for x in left..right {
                let position = matrix::Vec2(x as f32, y as f32);
                for candidate in &candidates {
                    if candidate.0.is_touching(position, candidate.1) {
                        hits
                            .entry(candidate.0.id)
                            .and_modify(|hit| {*hit += 1})
                            .or_insert(1);

                        break;
                    }
                }
            }
        }

        let mut hit: drawable::DrawableID = ID_NONE;
        let mut highest_hits: u32 = 0;

        for (id, num_hits) in hits.iter() {
            if *num_hits > highest_hits {
                hit = *id;
                highest_hits = *num_hits;
            }
        }

        hit
    }

    pub fn drawable_convex_hull_points(&mut self, drawable: drawable::DrawableID) -> Vec<f32> {
        let drawable = self.drawables.get(&drawable).expect("Drawable should exist");
        let silhouette = self.silhouettes.get(&drawable.silhouette).unwrap();

        let hull = convex_hull::calculate_drawable_convex_hull(drawable, silhouette);

        let mut points: Vec<f32> = Vec::new();

        for point in hull {
            points.push(point.0);
            points.push(point.1);
        }

        points
    }
}
