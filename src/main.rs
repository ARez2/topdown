#![deny(clippy::all)]
#![forbid(unsafe_code)]

use glam::{Vec3, Mat3};
use log::{debug, error};
use pixels::{Error, Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event::{Event, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;
use topdown::{drawable::Drawable, boxshape::BoxShape, color::Color};


const WIDTH: u32 = 300;
const HEIGHT: u32 = 200;
const SCALE: f64 = 3.0;

fn main() -> Result<(), Error> {
    env_logger::init();
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();

    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        let scaled_size = LogicalSize::new(WIDTH as f64 * SCALE, HEIGHT as f64 * SCALE);
        WindowBuilder::new()
            .with_title("Sandbox")
            .with_inner_size(scaled_size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture)?
    };

    let mut world = World::new(WIDTH as usize, HEIGHT as usize);


    event_loop.run(move |event, _, control_flow| {
        // The one and only event that winit_input_helper doesn't have for us...
        if let Event::RedrawRequested(_) = event {
            world.draw(pixels.get_frame_mut());
            if pixels
                .render()
                .map_err(|e| error!("pixels.render() failed: {}", e))
                .is_err()
            {
                *control_flow = ControlFlow::Exit;
                return;
            }
        };

        // For everything else, for let winit_input_helper collect events to build its state.
        // It returns `true` when it is time to update our game state and request a redraw.
        if input.update(&event) {
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            };
            // Resize the window
            if let Some(size) = input.window_resized() {
                pixels.resize_surface(size.width, size.height);
            };
            world.update();
            //if !paused || input.key_pressed_os(VirtualKeyCode::Space) {
            //    life.update();
            //}
            window.request_redraw();
        };

    });

    Ok(())
}


pub struct World {
    pub objects: Vec<Box<dyn Drawable>>,
    pub light_dir: Vec3,
    pub light_intensity: f32,
    pub rotation_x: f32,
    pub camera_pos: Vec3,
    pub width: usize,
    pub height: usize,
    pub time: f32,
}

impl World {
    pub fn new(width: usize, height: usize) -> World {


        World {
            objects: vec![
                Box::new(BoxShape::new(
                    Vec3::new(0.0, -10.0, 0.0),
                    Vec3::new(10.0, 10.0, 10.0),
                    Color::rgba(1.0, 0.3, 0.3, 1.0),
                )),
                Box::new(BoxShape::new(
                    Vec3::new(0.0, 0.0, 0.0),
                    Vec3::new(500.0, 1.0, 500.0),
                    Color::rgba(0.8, 0.8, 0.8, 1.0),
                )),
            ],
            light_dir: Vec3::new(-0.5, -0.5, 1.0),
            light_intensity: 1.0,
            rotation_x: 0.75,
            camera_pos: Vec3::new(-150.0, -100.0, 0.0),
            width,
            height,
            time: 0.0,
        }
    }

    fn grid_idx<I: std::convert::TryInto<usize>>(&self, x: I, y: I) -> Option<usize> {
        if let (Ok(x), Ok(y)) = (x.try_into(), y.try_into()) {
            if x < self.width && y < self.height {
                Some(x + y * self.width)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn screen_idx<I: std::convert::TryInto<usize>>(&self, x: I, y: I) -> Option<usize> {
        if let (Ok(x), Ok(y)) = (x.try_into(), y.try_into()) {
            if x < self.width && y < self.height {
                Some(x * 4 + y * self.width * 4)
            } else {
                None
            }
        } else {
            None
        }
    }


    fn is_in_bounds(&self, pt: Vec3) -> bool {
        (pt.x >= -1.0 && pt.x <= WIDTH as f32)
        && (pt.y >= -1.0 && pt.y <= HEIGHT as f32)
    }

    pub fn update(&mut self) {
        self.time += 0.04;
    }

    pub fn draw(&self, screen: &mut [u8]) {
        screen.fill(0);

        // (2D Position, Color, Normal)
        let mut all_points = Vec::<(Vec3, Color, Vec3)>::new();

        for object in self.objects.iter() {
            let origin = Vec3::new(0.0, 0.0, 0.0);//
            for (point, color, mut normal) in object.get_points() {
                //normal = self.rotateY(normal, self.time);
                // normal += origin;
                let mut pt_rot = self.rotateY(*point - origin, self.time);
                pt_rot += origin;
                //let mut pt_rot = *point;
                
                pt_rot = self.rotateX(pt_rot - origin, self.rotation_x);
                pt_rot += origin;
                //normal = self.rotateX(normal, self.rotation_x);
                //normal += origin;
                
                pt_rot -= self.camera_pos;
                let pos_2d = self.project(pt_rot);
                if self.is_in_bounds(pos_2d) {
                    all_points.push((pos_2d, *color, normal));
                };
            };
        };

        all_points.sort_by(|a, b| {
            if a.0.z < b.0.z {
                return std::cmp::Ordering::Less;
            } else if a.0.z > b.0.z {
                return std::cmp::Ordering::Greater;
            } else {
                return std::cmp::Ordering::Equal;
            };
        });
        let light_dir_rot = self.rotateY(self.light_dir, 0.0);
        let mut prev_normal = Vec3::ZERO;
        let mut prev_ratio = 0.0;
        for (pos_2d, color, normal) in all_points {
            
            let basecolor = color;
            let normal_ratio = match normal == prev_normal {
                true => prev_ratio,
                false => {
                    let r = normal.dot(light_dir_rot * 1.0) * self.light_intensity;
                    r.min(1.0).max(0.0)
                },
            };

            let mut lighted_color = basecolor;
            lighted_color *= normal_ratio;
            
            let idx = self.screen_idx(pos_2d.x.round() as usize, pos_2d.y.round() as usize);

            if let Some(idx) = idx {
                let c = lighted_color.as_255();
                screen[idx + 0] = c.r as u8;
                screen[idx + 1] = c.g as u8;
                screen[idx + 2] = c.b as u8;
                screen[idx + 3] = c.a as u8;
            };

            prev_normal = normal;
            prev_ratio = normal_ratio;
        }
        
    }


    pub fn project(&self, point: Vec3) -> Vec3 {
        Mat3::IDENTITY * point
    }


    pub fn rotateX(&self, point: Vec3, angle: f32) -> Vec3 {
        Mat3::from_cols(
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, angle.cos(), -angle.sin()),
            Vec3::new(0.0, angle.sin(), angle.cos()),
        ) * point
    }

    pub fn rotateY(&self, point: Vec3, angle: f32) -> Vec3 {
        Mat3::from_cols(
            Vec3::new(angle.cos(), 0.0, angle.sin()),
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(-angle.sin(), 0.0, angle.cos()),
        ) * point
    }

    pub fn rotateZ(&self, point: Vec3, angle: f32) -> Vec3 {
        Mat3::from_cols(
            Vec3::new(angle.cos(), -angle.sin(), 0.0),
            Vec3::new(angle.sin(), angle.cos(), 0.0),
            Vec3::new(0.0, 0.0, 1.0),
        ) * point
    }
}