pub type Mat4 = [f32; 16];
pub type Vec2 = (f32, f32);

trait Matrix {
    fn inverse(&self) -> Self;
}

impl Matrix for Mat4 {
    fn inverse(&self) -> Mat4 {
        unimplemented!()
    }
}
