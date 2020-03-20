use crate::matrix::*;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    pub type JSEffectMap;

    #[wasm_bindgen(method, getter)]
    pub fn u_color(this: &JSEffectMap) -> f64;
    #[wasm_bindgen(method, getter)]
    pub fn u_fisheye(this: &JSEffectMap) -> f64;
    #[wasm_bindgen(method, getter)]
    pub fn u_whirl(this: &JSEffectMap) -> f64;
    #[wasm_bindgen(method, getter)]
    pub fn u_pixelate(this: &JSEffectMap) -> f64;
    #[wasm_bindgen(method, getter)]
    pub fn u_mosaic(this: &JSEffectMap) -> f64;
    #[wasm_bindgen(method, getter)]
    pub fn u_brightness(this: &JSEffectMap) -> f64;
    #[wasm_bindgen(method, getter)]
    pub fn u_ghost(this: &JSEffectMap) -> f64;
}

#[derive(Default)]
pub struct Effects {
    pub color: f32,
    pub fisheye: f32,
    pub whirl: f32,
    pub pixelate: f32,
    pub mosaic: f32,
    pub brightness: f32,
    pub ghost: f32,
}

pub type EffectBits = u32;
pub enum EffectBitfield {
    Color = 0,
    Fisheye = 1,
    Whirl = 2,
    Pixelate = 3,
    Mosaic = 4,
    Brightness = 5,
    Ghost = 6,
}

pub const DISTORTION_EFFECT_MASK: EffectBits =
    1 << (EffectBitfield::Fisheye as u32) |
    1 << (EffectBitfield::Whirl as u32) |
    1 << (EffectBitfield::Pixelate as u32) |
    1 << (EffectBitfield::Mosaic as u32);

impl Effects {
    pub fn set_from_js(&mut self, effects: JSEffectMap) {
        self.color = effects.u_color() as f32;
        self.fisheye = effects.u_fisheye() as f32;
        self.whirl = effects.u_whirl() as f32;
        self.pixelate = effects.u_pixelate() as f32;
        self.mosaic = effects.u_mosaic() as f32;
        self.brightness = effects.u_brightness() as f32;
        self.ghost = effects.u_ghost() as f32;
    }
}

const CENTER: Vec2 = Vec2(0.5, 0.5);

pub fn transform_point(point: Vec2, effects: &Effects, effect_bits: &EffectBits, skin_size: Vec2) -> Vec2 {
    let mut out = point;

    if effect_bits & (1 << (EffectBitfield::Mosaic as u32)) != 0 {
        /*texcoord0 = fract(u_mosaic * texcoord0);*/
        out = Vec2(
            f32::fract(effects.mosaic * out.0),
            f32::fract(effects.mosaic * out.1)
        );
    }

    if effect_bits & (1 << (EffectBitfield::Pixelate as u32)) != 0 {
        /*vec2 pixelTexelSize = u_skinSize / u_pixelate;
        texcoord0 = (floor(texcoord0 * pixelTexelSize) + kCenter) / pixelTexelSize;*/
        let pixel_texel_size_x = skin_size.0 / effects.pixelate;
        let pixel_texel_size_y = skin_size.1 / effects.pixelate;

        out = Vec2(
            (f32::floor(out.0 * pixel_texel_size_x) + CENTER.0) / pixel_texel_size_x,
            (f32::floor(out.1 * pixel_texel_size_y) + CENTER.1) / pixel_texel_size_y
        );
    }

    if effect_bits & (1 << (EffectBitfield::Whirl as u32)) != 0 {
        /*const float kRadius = 0.5;
        vec2 offset = texcoord0 - kCenter;
        float offsetMagnitude = length(offset);
        float whirlFactor = max(1.0 - (offsetMagnitude / kRadius), 0.0);
        float whirlActual = u_whirl * whirlFactor * whirlFactor;
        float sinWhirl = sin(whirlActual);
        float cosWhirl = cos(whirlActual);
        mat2 rotationMatrix = mat2(
            cosWhirl, -sinWhirl,
            sinWhirl, cosWhirl
        );

        texcoord0 = rotationMatrix * offset + kCenter;*/

        const RADIUS: f32 = 0.5;
        let offset = out - CENTER;
        let offset_magnitude = offset.length();
        let whirl_factor = f32::max(1.0 - (offset_magnitude / RADIUS), 0.0);
        let whirl_actual = effects.whirl * whirl_factor * whirl_factor;
        let (sin_whirl, cos_whirl) = f32::sin_cos(whirl_actual);

        // texcoord0 = rotationMatrix * offset + kCenter;
        out.0 = (cos_whirl * offset.0) + (sin_whirl * offset.1) + CENTER.0;
        out.1 = (cos_whirl * offset.1) - (sin_whirl * offset.0) + CENTER.1;
    }

    if effect_bits & (1 << (EffectBitfield::Fisheye as u32)) != 0 {
        /* vec2 vec = (texcoord0 - kCenter) / kCenter;
        float vecLength = length(vec);
        float r = pow(min(vecLength, 1.0), u_fisheye) * max(1.0, vecLength);
        vec2 unit = vec / vecLength;

        texcoord0 = kCenter + r * unit * kCenter;*/

        let v = (out - CENTER) / CENTER;

        let len = v.length();
        let r = f32::powf(f32::min(len, 1.0), effects.fisheye) * f32::max(1.0, len);
        let unit: Vec2 = v / Vec2(len, len);

        out = CENTER + Vec2(r, r) * unit * CENTER;
    }

    out
}
