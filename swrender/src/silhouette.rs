use crate::matrix::Vec2;

pub type SilhouetteID = u32;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace = console)]
    pub fn time(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    pub fn timeEnd(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}


pub struct Silhouette {
    pub id: SilhouetteID,
    pub width: u32,
    pub height: u32,
    pub nominal_size: Vec2,
    data: Box<[u8]>,
    _blank: [u8; 4]
}

impl Silhouette {
    pub fn new(id: SilhouetteID) -> Silhouette {
        Silhouette {
            id,
            width: 0,
            height: 0,
            nominal_size: Vec2(0f32, 0f32),
            data: Box::new([0, 0, 0, 0]),
            _blank: [0, 0, 0, 0]
        }
    }

    pub fn set_data(&mut self, w: u32, h: u32, mut data: Box<[u8]>, nominal_size: Vec2, premultiplied: bool) {
        assert_eq!(data.len(), (w * h * 4) as usize, "silhouette data is improperly sized");

        self.width = w;
        self.height = h;
        self.nominal_size = nominal_size;

        if !premultiplied {
            let pixels = (*data).chunks_mut(4);

            for pixel in pixels {
                // This is indeed one branch per pixel. However, the branch predictor does a pretty good job of
                // eliminating branch overhead and this saves us several instructions per pixel.
                if pixel[3] == 0u8 {continue}

                let alpha = (pixel[3] as f32) / 255f32;

                pixel[0] = ((pixel[0] as f32) * alpha) as u8;
                pixel[1] = ((pixel[1] as f32) * alpha) as u8;
                pixel[2] = ((pixel[2] as f32) * alpha) as u8;
            }
        }

        self.data = data;
    }

    pub fn get_point(&self, x: i32, y: i32) -> bool {
        if x < 0 || y < 0 || (x as u32) >= self.width || (y as u32) >= self.height {
            false
        } else {
            let idx = (((y as u32 * self.width) + x as u32) * 4) as usize;
            self.data[idx+3] != 0u8
        }
    }

    pub fn get_color(&self, x: i32, y: i32) -> [u8; 4] {
        if x < 0 || y < 0 || (x as u32) >= self.width || (y as u32) >= self.height {
            self._blank
        } else {
            let idx = (((y as u32 * self.width) + x as u32) * 4) as usize;
            [self.data[idx], self.data[idx + 1], self.data[idx + 2], self.data[idx + 3]]
        }
    }

    pub fn is_touching_nearest(&self, vec: Vec2) -> bool {
        self.get_point((vec.0 * self.width as f32) as i32, (vec.1 * self.height as f32) as i32)
    }

    pub fn color_at_nearest(&self, vec: Vec2) -> [u8; 4] {
        self.get_color((vec.0 * self.width as f32) as i32, (vec.1 * self.height as f32) as i32)
    }

    pub fn is_touching_linear(&self, vec: Vec2) -> bool {
        let x = ((vec.0 * self.width as f32) - 0.5) as i32;
        let y = ((vec.1 * self.height as f32) - 0.5) as i32;

        self.get_point(x, y) ||
        self.get_point(x + 1, y) ||
        self.get_point(x, y + 1) ||
        self.get_point(x + 1, y + 1)
    }
}
