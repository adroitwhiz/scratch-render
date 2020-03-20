use crate::matrix::Vec2;

pub type SilhouetteID = u32;

pub struct Silhouette {
    pub id: SilhouetteID,
    pub width: u32,
    pub height: u32,
    pub nominal_size: Vec2,
    data: Box<[u8]>,
    _blank: Box<[u8; 4]>
}

impl Silhouette {
    pub fn new(id: SilhouetteID) -> Silhouette {
        Silhouette {
            id,
            width: 0,
            height: 0,
            nominal_size: Vec2(0f32, 0f32),
            data: Box::new([0, 0, 0, 0]),
            _blank: Box::new([0, 0, 0, 0])
        }
    }

    pub fn set_data(&mut self, w: u32, h: u32, data: Box<[u8]>, nominal_size: Vec2) {
        assert_eq!(data.len(), (w * h * 4) as usize, "silhouette data is improperly sized");

        self.width = w;
        self.height = h;
        self.data = data;
        self.nominal_size = nominal_size;
    }

    pub fn get_point(&self, x: i32, y: i32) -> bool {
        if x < 0 || y < 0 || (x as u32) >= self.width || (y as u32) >= self.height {
            false
        } else {
            let idx = (((y as u32 * self.width) + x as u32) * 4) as usize;
            self.data[idx+3] != 0u8
        }
    }

    pub fn get_color(&self, x: i32, y: i32) -> &[u8] {
        if x < 0 || y < 0 || (x as u32) >= self.width || (y as u32) >= self.height {
            &self._blank[0..4]
        } else {
            let idx = (((y as u32 * self.width) + x as u32) * 4) as usize;
            &self.data[idx..idx+4]
        }
    }
}
