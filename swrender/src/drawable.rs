use crate::effect_transform::{
    transform_color, transform_point, EffectBits, Effects, COLOR_EFFECT_MASK,
    DISTORTION_EFFECT_MASK,
};
use crate::matrix::*;
use crate::silhouette::*;

pub type DrawableID = i32;

/// The software-renderer version of a Drawable.
/// The `id` matches up with the corresponding JS-world Drawable.
pub struct Drawable {
    pub id: DrawableID,
    pub inverse_matrix: Mat4,
    pub silhouette: SilhouetteID,
    pub effects: Effects,
    pub effect_bits: EffectBits,
    pub use_nearest_neighbor: bool,
}

impl Drawable {
    /// Convert a "Scratch-space" location into a texture-space (0-1) location.
    pub fn get_local_position(&self, vec: Vec2) -> Vec2 {
        let v0 = vec.0 + 0.5;
        let v1 = vec.1 + 0.5;
        let m = self.inverse_matrix;
        let d = (v0 * m[3]) + (v1 * m[7]) + m[15];
        // The RenderWebGL quad flips the texture's X axis. So rendered bottom
        // left is 1, 0 and the top right is 0, 1. Flip the X axis so
        // localPosition matches that transformation.
        let out_x = 0.5 - (((v0 * m[0]) + (v1 * m[4]) + m[12]) / d);
        let out_y = (((v0 * m[1]) + (v1 * m[5]) + m[13]) / d) + 0.5;

        Vec2(out_x, out_y)
    }

    fn get_transformed_position(&self, vec: Vec2, skin_size: Vec2) -> Vec2 {
        if (self.effect_bits & DISTORTION_EFFECT_MASK) == 0 {
            vec
        } else {
            transform_point(vec, &self.effects, self.effect_bits, skin_size)
        }
    }

    /// Check if the "Scratch-space" position touches the passed silhouette.
    #[inline(always)]
    pub fn is_touching(&self, position: Vec2, silhouette: &Silhouette) -> bool {
        let local_position = self.get_local_position(position);
        if local_position.0 < 0f32
            || local_position.0 >= 1f32
            || local_position.1 < 0f32
            || local_position.1 >= 1f32
        {
            return false;
        }
        let local_position = self.get_transformed_position(local_position, silhouette.nominal_size);

        if self.use_nearest_neighbor {
            silhouette.is_touching_nearest(local_position)
        } else {
            silhouette.is_touching_linear(local_position)
        }
    }

    /// Sample a color from the given "Scratch-space" position of the passed silhouette.
    #[inline(always)]
    pub fn sample_color<'a>(&self, position: Vec2, silhouette: &'a Silhouette) -> [u8; 4] {
        let local_position = self.get_local_position(position);
        if local_position.0 < 0f32
            || local_position.0 >= 1f32
            || local_position.1 < 0f32
            || local_position.1 >= 1f32
        {
            return [0, 0, 0, 0];
        }
        let local_position = self.get_transformed_position(local_position, silhouette.nominal_size);

        // TODO: linear sampling
        let color = if self.use_nearest_neighbor {
            silhouette.color_at_nearest(local_position)
        } else {
            silhouette.color_at_nearest(local_position)
        };

        if (self.effect_bits & COLOR_EFFECT_MASK) == 0 {
            color
        } else {
            transform_color(color, &self.effects, self.effect_bits)
        }
    }
}
