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
use topdown::{drawable::Drawable, boxshape::BoxShape};


const WIDTH: u32 = 100;
const HEIGHT: u32 = 100;
const SCALE: f64 = 10.0;

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
    pub width: usize,
    pub height: usize,
    pub time: f32,
}

impl World {
    pub fn new(width: usize, height: usize) -> World {


        World {
            objects: vec![
                Box::new(BoxShape::new())
            ],
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


    pub fn update(&mut self) {
        self.time += 0.04;
    }

    pub fn draw(&self, screen: &mut [u8]) {
        screen.fill(0);

        for object in self.objects.iter() {
            let origin = object.get_origin();
            for (point, color) in object.get_points() {
                let mut pt_rot = self.rotateY(*point - origin, self.time);
                pt_rot += origin;
                pt_rot = self.rotateX(pt_rot - origin, 0.5);
                pt_rot += origin;

                let pos_2d = self.project(pt_rot);
                let idx = self.screen_idx(pos_2d.x.round() as usize, pos_2d.y.round() as usize);            

                if let Some(idx) = idx {
                    let c = color.as_255();
                    screen[idx + 0] = c.r as u8;
                    screen[idx + 1] = c.g as u8;
                    screen[idx + 2] = c.b as u8;
                    screen[idx + 3] = c.a as u8;
                };
            };
        };
        
    }

    // fn set_line(&mut self, mut start: Vec3, mut end: Vec3) {
    //     // probably should do sutherland-hodgeman if this were more serious.
    //     // instead just clamp the start pos, and draw until moving towards the
    //     // end pos takes us out of bounds.
    //     start = start.round();
    //     end = end.round();
    //     //let x0 = x0.max(0).min(self.width as isize);
    //     //let y0 = y0.max(0).min(self.height as isize);
    //     for (x, y, z) in line_drawing::Bresenham3d::new((start.x as i32, start.y as i32, start.z as i32), (end.x as i32, end.y as i32, end.z as i32)) {
    //         if let Some(i) = self.grid_idx(x, y) {
    //             self.cells.push(Cell{color: (255, 0, 0, 255), pos: Vec3::new(x as f32, y as f32, z as f32)});
    //         } else {
    //             break;
    //         }
    //     }
    // }

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