use glam::{Vec3};

use crate::{Drawable, Color, drawutil::{set_line, fill_vertical, fill_horizontal}};

#[derive(Clone, Copy)]
enum BoxPt {
    TopFrontL,
    TopFrontR,
    TopBackL,
    TopBackR,
    
    BottomFrontL,
    BottomFrontR,
    BottomBackL,
    BottomBackR,
}

#[derive(Clone, Copy)]
pub enum BoxFace {
    Up,
    Down,
    Left,
    Right,
    Front,
    Back,
}


pub struct BoxShape {
    pub points: Vec<(Vec3, Color, Vec3)>,
    pub pos: Vec3,
    pub scale: Vec3,
    
    normals: Vec<Vec3>,
}

impl BoxShape {
    pub fn new(pos: Vec3, scale: Vec3, color: Color) -> Self {
        let mut normals = Vec::<Vec3>::new();
        // We have 6 Faces
        normals.resize(6, Vec3::new(0.0, 0.0, 0.0));
        normals[BoxFace::Up as usize] = Vec3::new(0.0, -1.0, 0.0);
        normals[BoxFace::Down as usize] = Vec3::new(0.0, 1.0, 0.0);
        normals[BoxFace::Front as usize] = Vec3::new(0.0, 0.0, -1.0);
        normals[BoxFace::Back as usize] = Vec3::new(0.0, 0.0, 1.0);
        normals[BoxFace::Left as usize] = Vec3::new(-1.0, 0.0, 0.0);
        normals[BoxFace::Right as usize] = Vec3::new(1.0, 0.0, 0.0);

        
        let mut points = Vec::<(Vec3, Color, Vec3)>::new();
        points.resize(8, (Vec3::new(0.0, 0.0, 0.0), Color::black(), Vec3::new(0.0, 0.0, 0.0)));

        points[BoxPt::TopFrontL as usize] = (Vec3::new(0.0, 0.0, 0.0), color, normals[BoxFace::Up as usize]);
        points[BoxPt::TopFrontR as usize] = (Vec3::new(1.0, 0.0, 0.0), color, normals[BoxFace::Up as usize]);
        points[BoxPt::TopBackL as usize] = (Vec3::new(0.0, 0.0, 1.0), color, normals[BoxFace::Up as usize]);
        points[BoxPt::TopBackR as usize] = (Vec3::new(1.0, 0.0, 1.0), color, normals[BoxFace::Up as usize]);

        points[BoxPt::BottomFrontL as usize] = (Vec3::new(0.0, 1.0, 0.0), color, normals[BoxFace::Down as usize]);
        points[BoxPt::BottomFrontR as usize] = (Vec3::new(1.0, 1.0, 0.0), color, normals[BoxFace::Down as usize]);
        points[BoxPt::BottomBackL as usize] = (Vec3::new(0.0, 1.0, 1.0), color, normals[BoxFace::Down as usize]);
        points[BoxPt::BottomBackR as usize] = (Vec3::new(1.0, 1.0, 1.0), color, normals[BoxFace::Down as usize]);
        for pt in points.iter_mut() {
            pt.0 -= 0.5;
            pt.0 *= scale;
            pt.0 += pos;
        };
        
        for pt in fill_horizontal(points[BoxPt::BottomBackR as usize].0, points[BoxPt::BottomFrontL as usize].0, color) {
            points.push((pt.0, pt.1, normals[BoxFace::Down as usize]));
        };
        for pt in fill_vertical(points[BoxPt::TopFrontL as usize].0, points[BoxPt::BottomFrontR as usize].0, color) {
            points.push((pt.0, pt.1, normals[BoxFace::Front as usize]));
        };
        for pt in fill_vertical(points[BoxPt::TopBackL as usize].0, points[BoxPt::BottomBackR as usize].0, color) {
            points.push((pt.0, pt.1, normals[BoxFace::Back as usize]));
        };
        for pt in fill_vertical(points[BoxPt::TopFrontL as usize].0, points[BoxPt::BottomBackL as usize].0, color) {
            points.push((pt.0, pt.1, normals[BoxFace::Left as usize]));
        };
        for pt in fill_vertical(points[BoxPt::TopFrontR as usize].0, points[BoxPt::BottomBackR as usize].0, color) {
            points.push((pt.0, pt.1, normals[BoxFace::Right as usize]));
        };
        for pt in fill_horizontal(points[BoxPt::TopBackR as usize].0, points[BoxPt::TopFrontL as usize].0, color) {
            points.push((pt.0, pt.1, normals[BoxFace::Up as usize]));
        };

        
        BoxShape {
            points,
            pos,
            scale,
            
            normals,
        }
    }
}


impl Drawable for BoxShape {
    fn get_points(&self) -> &Vec<(Vec3, Color, Vec3)> {
        &self.points
    }

    fn get_origin(&self) -> Vec3 {
        if self.points.len() == 0 {
            return Vec3::new(0.0, 0.0, 0.0);
        };
        let mut sum = Vec3::new(0.0, 0.0, 0.0);
        let mut i = 0;
        for pt in self.points.iter() {
            if i == 8 {
                break;
            };
            sum += pt.0;
            i += 1;
        };
        sum / 8.0
    }
}