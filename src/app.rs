use log::error;
use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event::{Event, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};
use winit_input_helper::WinitInputHelper;

use crate::worldable::Worldable;

pub struct App {
    event_loop: EventLoop<()>,
    input: WinitInputHelper,
    window: Window,
    pixels: Pixels,
    world: Box<dyn Worldable>,
}

impl App {
    pub fn new(window_width: u32, window_height: u32, world: Box<dyn Worldable>) -> Self {
        let event_loop = EventLoop::new();
        let input = WinitInputHelper::new();
        let window = {
            let size = LogicalSize::new(window_width as f64, window_height as f64);
            WindowBuilder::new()
                .with_title("Hello Pixels")
                .with_inner_size(size)
                .with_min_inner_size(size)
                .build(&event_loop)
                .unwrap()
        };

        let pixels = {
            let window_size = window.inner_size();
            let surface_texture =
                SurfaceTexture::new(window_size.width, window_size.height, &window);
            Pixels::new(window_width, window_height, surface_texture).unwrap()
        };

        Self {
            event_loop,
            input,
            window,
            pixels,
            world,
        }
    }

    pub fn run(mut self) {
        self.world.setup();

        self.event_loop.run(move |event, _, control_flow| {
            // Draw the current frame
            if let Event::RedrawRequested(_) = event {
                self.world.draw(self.pixels.get_frame());
                if self
                    .pixels
                    .render()
                    .map_err(|e| error!("pixels.render() failed: {}", e))
                    .is_err()
                {
                    *control_flow = ControlFlow::Exit;
                    return;
                }
            }

            // Handle input events
            if self.input.update(&event) {
                // Close events
                if self.input.key_pressed(VirtualKeyCode::Escape) || self.input.quit() {
                    *control_flow = ControlFlow::Exit;
                    return;
                }

                // Resize the window
                if let Some(size) = self.input.window_resized() {
                    self.pixels.resize_surface(size.width, size.height);
                    self.world.resized(size.width, size.height);
                }

                if self.input.mouse_pressed(0) {
                    let mouse_position = self.input.mouse().unwrap();
                    let pixel_position = self.pixels.window_pos_to_pixel(mouse_position);
                    let (clamped_x, clamped_y) =
                        pixel_position.unwrap_or_else(|pos| self.pixels.clamp_pixel_pos(pos));
                    self.world.mouse_press((clamped_x as u32, clamped_y as u32));
                }

                self.world.key_press(&self.input);

                // Update internal state and request a redraw
                self.world.update();
                self.window.request_redraw();
            }
        });
    }
}
