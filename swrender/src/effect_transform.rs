use crate::matrix::*;

use std::f32;
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

pub const COLOR_EFFECT_MASK: EffectBits = 1 << (EffectBitfield::Color as u32)
    | 1 << (EffectBitfield::Brightness as u32)
    | 1 << (EffectBitfield::Ghost as u32);

pub const DISTORTION_EFFECT_MASK: EffectBits = 1 << (EffectBitfield::Fisheye as u32)
    | 1 << (EffectBitfield::Whirl as u32)
    | 1 << (EffectBitfield::Pixelate as u32)
    | 1 << (EffectBitfield::Mosaic as u32);

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

/// Converts an RGB color value to HSV. Conversion formula
/// adapted from http://lolengine.net/blog/2013/01/13/fast-rgb-to-hsv.
/// Assumes all channels are in the range [0, 1].
fn rgb_to_hsv(r: f32, g: f32, b: f32) -> (f32, f32, f32) {
    let mut r = r;
    let mut g = g;
    let mut b = b;

    let mut tmp: f32;

    let mut k = 0f32;

    if g < b {
        tmp = g;
        g = b;
        b = tmp;
        k = -1f32;
    }

    if r < g {
        tmp = g;
        g = r;
        r = tmp;
        k = (-2f32 / 6f32) - k;
    }

    let chroma = r - f32::min(g, b);

    let h = f32::abs(k + (g - b) / (6f32 * chroma + f32::EPSILON));
    let s = chroma / (r + f32::EPSILON);
    let v = r;

    (h, s, v)
}

/// Converts an HSV color value to RRB. Conversion formula
/// adapted from https://gist.github.com/mjackson/5311256.
/// Assumes all channels are in the range [0, 1].
fn hsv_to_rgb(h: f32, s: f32, v: f32) -> (f32, f32, f32) {
    if s < 1e-18 {
        return (v, v, v);
    }

    let i = (h * 6f32).floor();
    let f = (h * 6f32) - i;
    let p = v * (1f32 - s);
    let q = v * (1f32 - (s * f));
    let t = v * (1f32 - (s * (1f32 - f)));

    match i as u32 {
        0 => (v, t, p),
        1 => (q, v, p),
        2 => (p, v, t),
        3 => (p, q, v),
        4 => (t, p, v),
        5 => (v, p, q),
        _ => unreachable!(),
    }
}

/// Transform a color in-place according to the passed effects + effect bits.  Will apply
/// Ghost and Color and Brightness effects.
pub fn transform_color<'a>(color: [u8; 4], effects: &Effects, effect_bits: EffectBits) -> [u8; 4] {
    const COLOR_DIVISOR: f32 = 1f32 / 255f32;
    let mut rgba: [f32; 4] = [
        (color[0] as f32) * COLOR_DIVISOR,
        (color[1] as f32) * COLOR_DIVISOR,
        (color[2] as f32) * COLOR_DIVISOR,
        (color[3] as f32) * COLOR_DIVISOR,
    ];

    let enable_color = effect_bits & (1 << (EffectBitfield::Color as u32)) != 0;
    let enable_brightness = effect_bits & (1 << (EffectBitfield::Brightness as u32)) != 0;

    if enable_brightness || enable_color {
        let alpha = rgba[3] + f32::EPSILON;
        rgba[0] /= alpha;
        rgba[1] /= alpha;
        rgba[2] /= alpha;

        if enable_color {
            /*vec3 hsv = convertRGB2HSV(gl_FragColor.xyz);

            // this code forces grayscale values to be slightly saturated
            // so that some slight change of hue will be visible
            const float minLightness = 0.11 / 2.0;
            const float minSaturation = 0.09;
            if (hsv.z < minLightness) hsv = vec3(0.0, 1.0, minLightness);
            else if (hsv.y < minSaturation) hsv = vec3(0.0, minSaturation, hsv.z);

            hsv.x = mod(hsv.x + u_color, 1.0);
            if (hsv.x < 0.0) hsv.x += 1.0;

            gl_FragColor.rgb = convertHSV2RGB(hsv);*/

            let (mut h, mut s, mut v) = rgb_to_hsv(rgba[0], rgba[1], rgba[2]);

            const MIN_LIGHTNESS: f32 = 0.11 / 2f32;
            const MIN_SATURATION: f32 = 0.09;

            if v < MIN_LIGHTNESS {
                v = MIN_LIGHTNESS
            } else if s < MIN_SATURATION {
                s = MIN_SATURATION
            }

            h = f32::fract(h + effects.color);

            let (r, g, b) = hsv_to_rgb(h, s, v);
            rgba[0] = r;
            rgba[1] = g;
            rgba[2] = b;
        }

        if enable_brightness {
            // gl_FragColor.rgb = clamp(gl_FragColor.rgb + vec3(u_brightness), vec3(0), vec3(1));
            rgba[0] = (rgba[0] + effects.brightness).min(1f32).max(0f32);
            rgba[1] = (rgba[1] + effects.brightness).min(1f32).max(0f32);
            rgba[2] = (rgba[2] + effects.brightness).min(1f32).max(0f32);
        }

        rgba[0] *= alpha;
        rgba[1] *= alpha;
        rgba[2] *= alpha;
    }

    // gl_FragColor *= u_ghost
    if effect_bits & (1 << (EffectBitfield::Ghost as u32)) != 0 {
        rgba[0] *= effects.ghost;
        rgba[1] *= effects.ghost;
        rgba[2] *= effects.ghost;
        rgba[3] *= effects.ghost;
    }

    [
        (rgba[0] * 255f32) as u8,
        (rgba[1] * 255f32) as u8,
        (rgba[2] * 255f32) as u8,
        (rgba[3] * 255f32) as u8,
    ]
}

/// Transform a texture coordinate to one that would be used after applying shader effects.
pub fn transform_point(
    point: Vec2,
    effects: &Effects,
    effect_bits: EffectBits,
    skin_size: Vec2,
) -> Vec2 {
    const CENTER: Vec2 = Vec2(0.5, 0.5);

    let mut out = point;

    if effect_bits & (1 << (EffectBitfield::Mosaic as u32)) != 0 {
        /*texcoord0 = fract(u_mosaic * texcoord0);*/
        out = Vec2(
            f32::fract(effects.mosaic * out.0),
            f32::fract(effects.mosaic * out.1),
        );
    }

    if effect_bits & (1 << (EffectBitfield::Pixelate as u32)) != 0 {
        /*vec2 pixelTexelSize = u_skinSize / u_pixelate;
        texcoord0 = (floor(texcoord0 * pixelTexelSize) + kCenter) / pixelTexelSize;*/
        let pixel_texel_size_x = skin_size.0 / effects.pixelate;
        let pixel_texel_size_y = skin_size.1 / effects.pixelate;

        out = Vec2(
            (f32::floor(out.0 * pixel_texel_size_x) + CENTER.0) / pixel_texel_size_x,
            (f32::floor(out.1 * pixel_texel_size_y) + CENTER.1) / pixel_texel_size_y,
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
