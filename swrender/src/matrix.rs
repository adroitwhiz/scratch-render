use std::f32;
use std::ops;

pub type Mat4 = [f32; 16];

#[derive(Copy, Clone)]
pub struct Vec2(pub f32, pub f32);

impl ops::Add for Vec2 {
    type Output = Vec2;

    fn add(self, other: Vec2) -> Vec2 {
        Vec2(self.0 + other.0, self.1 + other.1)
    }
}

impl ops::Sub for Vec2 {
    type Output = Vec2;

    fn sub(self, other: Vec2) -> Vec2 {
        Vec2(self.0 - other.0, self.1 - other.1)
    }
}

impl ops::Mul for Vec2 {
    type Output = Vec2;

    fn mul(self, other: Vec2) -> Vec2 {
        Vec2(self.0 * other.0, self.1 * other.1)
    }
}

impl ops::Div for Vec2 {
    type Output = Vec2;

    fn div(self, other: Vec2) -> Vec2 {
        Vec2(self.0 / other.0, self.1 / other.1)
    }
}

impl ops::Neg for Vec2 {
    type Output = Vec2;

    fn neg(self) -> Vec2 {
        Vec2(-self.0, -self.1)
    }
}

impl Vec2 {
    pub fn length(&self) -> f32 {
        f32::sqrt(self.0 * self.0 + self.1 * self.1)
    }
}

pub trait Matrix {
    fn inverse(&self) -> Self;
}

impl Matrix for Mat4 {
    fn inverse(&self) -> Self {
        let m00 = self[0 * 4 + 0];
        let m01 = self[0 * 4 + 1];
        let m02 = self[0 * 4 + 2];
        let m03 = self[0 * 4 + 3];
        let m10 = self[1 * 4 + 0];
        let m11 = self[1 * 4 + 1];
        let m12 = self[1 * 4 + 2];
        let m13 = self[1 * 4 + 3];
        let m20 = self[2 * 4 + 0];
        let m21 = self[2 * 4 + 1];
        let m22 = self[2 * 4 + 2];
        let m23 = self[2 * 4 + 3];
        let m30 = self[3 * 4 + 0];
        let m31 = self[3 * 4 + 1];
        let m32 = self[3 * 4 + 2];
        let m33 = self[3 * 4 + 3];
        let tmp_0 = m22 * m33;
        let tmp_1 = m32 * m23;
        let tmp_2 = m12 * m33;
        let tmp_3 = m32 * m13;
        let tmp_4 = m12 * m23;
        let tmp_5 = m22 * m13;
        let tmp_6 = m02 * m33;
        let tmp_7 = m32 * m03;
        let tmp_8 = m02 * m23;
        let tmp_9 = m22 * m03;
        let tmp_10 = m02 * m13;
        let tmp_11 = m12 * m03;
        let tmp_12 = m20 * m31;
        let tmp_13 = m30 * m21;
        let tmp_14 = m10 * m31;
        let tmp_15 = m30 * m11;
        let tmp_16 = m10 * m21;
        let tmp_17 = m20 * m11;
        let tmp_18 = m00 * m31;
        let tmp_19 = m30 * m01;
        let tmp_20 = m00 * m21;
        let tmp_21 = m20 * m01;
        let tmp_22 = m00 * m11;
        let tmp_23 = m10 * m01;

        let t0: f32 =
            (tmp_0 * m11 + tmp_3 * m21 + tmp_4 * m31) - (tmp_1 * m11 + tmp_2 * m21 + tmp_5 * m31);
        let t1 =
            (tmp_1 * m01 + tmp_6 * m21 + tmp_9 * m31) - (tmp_0 * m01 + tmp_7 * m21 + tmp_8 * m31);
        let t2 =
            (tmp_2 * m01 + tmp_7 * m11 + tmp_10 * m31) - (tmp_3 * m01 + tmp_6 * m11 + tmp_11 * m31);
        let t3 =
            (tmp_5 * m01 + tmp_8 * m11 + tmp_11 * m21) - (tmp_4 * m01 + tmp_9 * m11 + tmp_10 * m21);

        let d = 1.0 / (m00 * t0 + m10 * t1 + m20 * t2 + m30 * t3);

        let mut dst: Mat4 = [0f32; 16];

        dst[0] = d * t0;
        dst[1] = d * t1;
        dst[2] = d * t2;
        dst[3] = d * t3;
        dst[4] = d
            * ((tmp_1 * m10 + tmp_2 * m20 + tmp_5 * m30)
                - (tmp_0 * m10 + tmp_3 * m20 + tmp_4 * m30));
        dst[5] = d
            * ((tmp_0 * m00 + tmp_7 * m20 + tmp_8 * m30)
                - (tmp_1 * m00 + tmp_6 * m20 + tmp_9 * m30));
        dst[6] = d
            * ((tmp_3 * m00 + tmp_6 * m10 + tmp_11 * m30)
                - (tmp_2 * m00 + tmp_7 * m10 + tmp_10 * m30));
        dst[7] = d
            * ((tmp_4 * m00 + tmp_9 * m10 + tmp_10 * m20)
                - (tmp_5 * m00 + tmp_8 * m10 + tmp_11 * m20));
        dst[8] = d
            * ((tmp_12 * m13 + tmp_15 * m23 + tmp_16 * m33)
                - (tmp_13 * m13 + tmp_14 * m23 + tmp_17 * m33));
        dst[9] = d
            * ((tmp_13 * m03 + tmp_18 * m23 + tmp_21 * m33)
                - (tmp_12 * m03 + tmp_19 * m23 + tmp_20 * m33));
        dst[10] = d
            * ((tmp_14 * m03 + tmp_19 * m13 + tmp_22 * m33)
                - (tmp_15 * m03 + tmp_18 * m13 + tmp_23 * m33));
        dst[11] = d
            * ((tmp_17 * m03 + tmp_20 * m13 + tmp_23 * m23)
                - (tmp_16 * m03 + tmp_21 * m13 + tmp_22 * m23));
        dst[12] = d
            * ((tmp_14 * m22 + tmp_17 * m32 + tmp_13 * m12)
                - (tmp_16 * m32 + tmp_12 * m12 + tmp_15 * m22));
        dst[13] = d
            * ((tmp_20 * m32 + tmp_12 * m02 + tmp_19 * m22)
                - (tmp_18 * m22 + tmp_21 * m32 + tmp_13 * m02));
        dst[14] = d
            * ((tmp_18 * m12 + tmp_23 * m32 + tmp_15 * m02)
                - (tmp_22 * m32 + tmp_14 * m02 + tmp_19 * m12));
        dst[15] = d
            * ((tmp_22 * m22 + tmp_16 * m02 + tmp_21 * m12)
                - (tmp_20 * m12 + tmp_23 * m22 + tmp_17 * m02));

        dst
    }
}
