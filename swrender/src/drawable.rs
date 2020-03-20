use crate::silhouette::*;
use crate::matrix::*;

pub type DrawableID = u32;

pub struct Drawable {
    pub matrix: Mat4,
    pub inverse_matrix: Mat4,
    pub silhouette: SilhouetteID,
    pub id: DrawableID
}

impl Drawable {
    pub fn get_local_position(&self, vec: Vec2) -> Vec2 {
        let v0 = vec.0 - 0.5;
        let v1 = vec.1 + 0.5;
        let m = self.inverse_matrix;
        let d = (v0 * m[3]) + (v1 * m[7]) + m[15];
        // The RenderWebGL quad flips the texture's X axis. So rendered bottom
        // left is 1, 0 and the top right is 0, 1. Flip the X axis so
        // localPosition matches that transformation.
        let out_x = 0.5 - (((v0 * m[0]) + (v1 * m[4]) + m[12]) / d);
        let out_y = (((v0 * m[1]) + (v1 * m[5]) + m[13]) / d) + 0.5;

        (out_x, out_y)
    }

    #[inline(always)]
    pub fn is_touching(&self, position: Vec2, silhouette: &Silhouette) -> bool {
        let local_position = self.get_local_position(position);
        silhouette.get_point((local_position.0 * silhouette.width as f32) as i32, (local_position.1 * silhouette.height as f32) as i32)
    }
}
