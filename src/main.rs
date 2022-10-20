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
use topdown::cell::Cell;


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
    pub cells: Vec<Cell>,
    pub width: usize,
    pub height: usize,
    pub time: f32,
}

impl World {
    pub fn new(width: usize, height: usize) -> World {
        let mut cells =  vec![];

        let scaleX = 15.0;
        let scaleY = 50.0;
        let scaleZ = 15.0;
        let off = 15.0;
        for z in 0..=1 {
            for y in 0..=1 {
                for x in 0..=1 {
                    cells.push(Cell {pos: Vec3::new(x as f32 * scaleX + off, y as f32 * scaleY + off, z as f32 * scaleZ + off), color: (255, 0, 0, 255)});
                };
            };
        };

        World {
            cells,
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
        self.set_line(self.cells[0].pos, self.cells[1].pos);
        self.set_line(self.cells[0].pos, self.cells[2].pos);
        self.set_line(self.cells[1].pos, self.cells[3].pos);
        self.set_line(self.cells[2].pos, self.cells[3].pos);

        self.set_line(self.cells[4].pos, self.cells[5].pos);
        self.set_line(self.cells[4].pos, self.cells[6].pos);
        //self.set_line(self.cells[5].pos, self.cells[6].pos);
        self.set_line(self.cells[5].pos, self.cells[7].pos);
    }

    pub fn draw(&self, screen: &mut [u8]) {
        screen.fill(0);

        for cell in self.cells.iter() {
            let pos2 = Vec3::new(cell.pos.x, cell.pos.y, cell.pos.z);
            let rot_pt = (7.5 + 15.0) * Vec3::new(1.0, 1.0, 1.0);
            let mut pos_rotated = self.rotateY(pos2 - rot_pt, self.time);
            pos_rotated += rot_pt;
            pos_rotated = self.rotateX(pos_rotated - rot_pt, 0.5);
            pos_rotated += rot_pt;
            let pos_2d = self.project(pos_rotated);
            let idx = self.screen_idx(pos_2d.x.round() as usize, pos_2d.y.round() as usize);            

            if let Some(idx) = idx {
                let c = cell.color;
                screen[idx + 0] = c.0;
                screen[idx + 1] = c.1;
                screen[idx + 2] = c.2;
                screen[idx + 3] = c.3;
            };
        };
        
    }

    fn set_line(&mut self, mut start: Vec3, mut end: Vec3) {
        // probably should do sutherland-hodgeman if this were more serious.
        // instead just clamp the start pos, and draw until moving towards the
        // end pos takes us out of bounds.
        start = start.round();
        end = end.round();
        //let x0 = x0.max(0).min(self.width as isize);
        //let y0 = y0.max(0).min(self.height as isize);
        for (x, y, z) in line_drawing::Bresenham3d::new((start.x as i32, start.y as i32, start.z as i32), (end.x as i32, end.y as i32, end.z as i32)) {
            if let Some(i) = self.grid_idx(x, y) {
                self.cells.push(Cell{color: (255, 0, 0, 255), pos: Vec3::new(x as f32, y as f32, z as f32)});
            } else {
                break;
            }
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