use glam::Vec3;

use crate::Color;



pub fn fill_vertical(start: Vec3, end: Vec3, color: Color) -> Vec<(Vec3, Color)> {
    let mut points = vec![];
    let min = (start.y.round() as usize).min(end.y.round() as usize);
    let max = (start.y.round() as usize).max(end.y.round() as usize);
    for y in min..=max {
        let mut linestart = start;
        linestart.y = y as f32;
        let mut lineend = end;
        lineend.y = y as f32;
        let linepts = set_line(linestart, lineend, color);
        for pt in linepts.iter() {
            points.push(*pt);
        };
    };
    points
}


pub fn fill_horizontal(start: Vec3, end: Vec3, color: Color) -> Vec<(Vec3, Color)> {
    let mut points = vec![];
    let min = (start.x.round() as usize).min(end.x.round() as usize);
    let max = (start.x.round() as usize).max(end.x.round() as usize);
    for x in min..=max {
        let mut linestart = start;
        linestart.x = x as f32;
        let mut lineend = end;
        lineend.x = x as f32;
        let linepts = set_line(linestart, lineend, color);
        for pt in linepts.iter() {
            points.push(*pt);
        };
    };
    points
}


pub fn set_line(mut start: Vec3, mut end: Vec3, color: Color) -> Vec<(Vec3, Color)> {
    let mut points = vec![];

    // probably should do sutherland-hodgeman if this were more serious.
    // instead just clamp the start pos, and draw until moving towards the
    // end pos takes us out of bounds.
    start = start.round();
    end = end.round();
    //let x0 = x0.max(0).min(self.width as isize);
    //let y0 = y0.max(0).min(self.height as isize);
    for (x, y, z) in line_drawing::Bresenham3d::new((start.x as i32, start.y as i32, start.z as i32), (end.x as i32, end.y as i32, end.z as i32)) {
        points.push(
            (
                Vec3::new(x as f32, y as f32, z as f32),
                color
            )
        );
    };
    points
}