use glam::Vec3;


#[derive(Clone, Copy, Debug, Default)]
pub struct Cell {
    pub color: (u8, u8, u8, u8),
    pub pos: Vec3,
    //pub scale: f32,
}

impl Cell {
    pub fn new(pos: Vec3) -> Self {
        Self {pos, color: (0, 0, 0, 255), }//scale: 1.0
    }
}