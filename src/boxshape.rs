use glam::{Vec3};

use crate::{Drawable, Color, drawutil::set_line};

enum BoxSides {
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
}

impl BoxShape {
    pub fn new() -> Self {
        let mut points = Vec::<(Vec3, Color)>::new();
        points.resize(8, (Vec3::new(0.0, 0.0, 0.0), Color::black()));

        let scaleX = 15.0;
        let scaleY = 50.0;
        let scaleZ = 15.0;
        let off = 15.0;
        let color = Color::rgba(1.0, 0.0, 0.0, 1.0);
        points[BoxSides::TopFrontL as usize] = (
            Vec3::new(0.0 * scaleX + off, 0.0 * scaleY + off, 0.0 * scaleZ + off),
            color
        );
        points[BoxSides::TopFrontR as usize] = (
            Vec3::new(1.0 * scaleX + off, 0.0 * scaleY + off, 0.0 * scaleZ + off),
            color
        );
        points[BoxSides::TopBackL as usize] = (
            Vec3::new(0.0 * scaleX + off, 0.0 * scaleY + off, 1.0 * scaleZ + off),
            color
        );
        points[BoxSides::TopBackR as usize] = (
            Vec3::new(1.0 * scaleX + off, 0.0 * scaleY + off, 1.0 * scaleZ + off),
            color
        );

        points[BoxSides::BottomFrontL as usize] = (
            Vec3::new(0.0 * scaleX + off, 1.0 * scaleY + off, 0.0 * scaleZ + off),
            color
        );
        points[BoxSides::BottomFrontR as usize] = (
            Vec3::new(1.0 * scaleX + off, 1.0 * scaleY + off, 0.0 * scaleZ + off),
            color
        );
        points[BoxSides::BottomBackL as usize] = (
            Vec3::new(0.0 * scaleX + off, 1.0 * scaleY + off, 1.0 * scaleZ + off),
            color
        );
        points[BoxSides::BottomBackR as usize] = (
            Vec3::new(1.0 * scaleX + off, 1.0 * scaleY + off, 1.0 * scaleZ + off),
            color
        );

        let tcol = Color::rgba(0.0, 1.0, 0.0, 1.0);
        for pt in 
        set_line(points[BoxSides::BottomFrontL as usize].0, points[BoxSides::BottomFrontR as usize].0, color).iter()
        .chain(set_line(points[BoxSides::BottomFrontL as usize].0, points[BoxSides::BottomBackL as usize].0, color).iter())
        .chain(set_line(points[BoxSides::BottomFrontR as usize].0, points[BoxSides::BottomBackR as usize].0, color).iter())
        .chain(set_line(points[BoxSides::BottomBackL as usize].0, points[BoxSides::BottomBackR as usize].0, color).iter())
        
        // Connection between top and bottom
        .chain(set_line(points[BoxSides::BottomFrontL as usize].0, points[BoxSides::TopFrontL as usize].0, color).iter())
        .chain(set_line(points[BoxSides::BottomFrontR as usize].0, points[BoxSides::TopFrontR as usize].0, color).iter())
        .chain(set_line(points[BoxSides::BottomBackL as usize].0, points[BoxSides::TopBackL as usize].0, color).iter())
        .chain(set_line(points[BoxSides::BottomBackR as usize].0, points[BoxSides::TopBackR as usize].0, color).iter())
        
        // Top face
        .chain(set_line(points[BoxSides::TopFrontL as usize].0, points[BoxSides::TopBackL as usize].0, tcol).iter())
        .chain(set_line(points[BoxSides::TopFrontR as usize].0, points[BoxSides::TopBackR as usize].0, tcol).iter())
        .chain(set_line(points[BoxSides::TopFrontL as usize].0, points[BoxSides::TopFrontR as usize].0, tcol).iter())
        .chain(set_line(points[BoxSides::TopBackL as usize].0, points[BoxSides::TopBackR as usize].0, tcol).iter())
            {
            points.push(*pt)
        };
        
        BoxShape {
            points,
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
        for pt in self.points.iter() {
            sum += pt.0;
        };
        sum / self.points.len() as f32
    }
}