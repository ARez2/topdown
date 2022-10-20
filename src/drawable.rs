use glam::{Vec3};
use crate::color::Color;

pub trait Drawable {
    fn get_points(&self) -> &Vec<(Vec3, Color, Vec3)>;
    fn get_origin(&self) -> Vec3;
}