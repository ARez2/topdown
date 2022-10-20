use glam::{Vec3};

use crate::{Drawable, Color, drawutil::{set_line, fill_vertical, fill_horizontal}};

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


pub struct BoxShape {
    pub points: Vec<(Vec3, Color)>,
    pub pos: Vec3,
    pub scale: Vec3,
}

impl BoxShape {
    pub fn new(pos: Vec3, scale: Vec3) -> Self {
        let mut points = Vec::<(Vec3, Color)>::new();
        points.resize(8, (Vec3::new(0.0, 0.0, 0.0), Color::black()));

        let scaleX = 15.0;
        let scaleY = 50.0;
        let scaleZ = 15.0;
        let off = 15.0;
        let color = Color::rgba(1.0, 0.0, 0.0, 1.0);
        points[BoxPt::TopFrontL as usize] = (Vec3::new(0.0, 0.0, 0.0), color);
        points[BoxPt::TopFrontR as usize] = (Vec3::new(1.0, 0.0, 0.0), color);
        points[BoxPt::TopBackL as usize] = (Vec3::new(0.0, 0.0, 1.0),color);
        points[BoxPt::TopBackR as usize] = (Vec3::new(1.0, 0.0, 1.0),color);

        points[BoxPt::BottomFrontL as usize] = (Vec3::new(0.0, 1.0, 0.0),color);
        points[BoxPt::BottomFrontR as usize] = (Vec3::new(1.0, 1.0, 0.0),color);
        points[BoxPt::BottomBackL as usize] = (Vec3::new(0.0, 1.0, 1.0),color);
        points[BoxPt::BottomBackR as usize] = (Vec3::new(1.0, 1.0, 1.0),color);
        for pt in points.iter_mut() {
            pt.0 *= scale;
            pt.0 += pos;
        };

        let tcol = Color::rgba(0.0, 1.0, 0.0, 1.0);
        let blue = Color::rgba(0.0, 0.0, 1.0, 1.0);
        
        
        for pt in fill_horizontal(points[BoxPt::BottomBackR as usize].0, points[BoxPt::BottomFrontL as usize].0, blue) {
            points.push(pt);
        };
        for pt in fill_vertical(points[BoxPt::TopFrontL as usize].0, points[BoxPt::BottomFrontR as usize].0, color) {
            points.push(pt);
        };
        for pt in fill_vertical(points[BoxPt::TopBackL as usize].0, points[BoxPt::BottomBackR as usize].0, tcol) {
            points.push(pt);
        };
        for pt in fill_horizontal(points[BoxPt::TopBackR as usize].0, points[BoxPt::TopFrontL as usize].0, blue) {
            points.push(pt);
        };

        
        // for pt in 
        // set_line(points[BoxPt::BottomFrontL as usize].0, points[BoxPt::BottomFrontR as usize].0, color).iter()
        // .chain(set_line(points[BoxPt::BottomFrontL as usize].0, points[BoxPt::BottomBackL as usize].0, color).iter())
        // .chain(set_line(points[BoxPt::BottomFrontR as usize].0, points[BoxPt::BottomBackR as usize].0, color).iter())
        // .chain(set_line(points[BoxPt::BottomBackL as usize].0, points[BoxPt::BottomBackR as usize].0, color).iter())
        
        // // Connection between top and bottom
        // .chain(set_line(points[BoxPt::BottomFrontL as usize].0, points[BoxPt::TopFrontL as usize].0, color).iter())
        // .chain(set_line(points[BoxPt::BottomFrontR as usize].0, points[BoxPt::TopFrontR as usize].0, color).iter())
        // .chain(set_line(points[BoxPt::BottomBackL as usize].0, points[BoxPt::TopBackL as usize].0, color).iter())
        // .chain(set_line(points[BoxPt::BottomBackR as usize].0, points[BoxPt::TopBackR as usize].0, color).iter())
        
        // // Top face
        // .chain(set_line(points[BoxPt::TopFrontL as usize].0, points[BoxPt::TopBackL as usize].0, tcol).iter())
        // .chain(set_line(points[BoxPt::TopFrontR as usize].0, points[BoxPt::TopBackR as usize].0, tcol).iter())
        // .chain(set_line(points[BoxPt::TopFrontL as usize].0, points[BoxPt::TopFrontR as usize].0, tcol).iter())
        // .chain(set_line(points[BoxPt::TopBackL as usize].0, points[BoxPt::TopBackR as usize].0, tcol).iter())
        //     {
        //     points.push(*pt)
        // };
        
        BoxShape {
            points,
            pos,
            scale,
        }
    }
}


impl Drawable for BoxShape {
    fn get_points(&self) -> &Vec<(Vec3, Color)> {
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