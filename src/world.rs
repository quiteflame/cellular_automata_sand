use std::cmp::min;

use rand::Rng;
use winit::event::VirtualKeyCode;
use winit_input_helper::WinitInputHelper;

use crate::{
    substance::{Substance, SubstanceKind},
    worldable::Worldable,
};

fn random_bool() -> bool {
    let mut rng = rand::thread_rng();
    rng.gen_range(0..10 as i32) < 5
}

pub struct World {
    pub elements: Vec<Substance>,
    scrach_elements: Vec<Substance>,
    width: u32,
    height: u32,
    drop_sand: bool,
    drop_water: bool,
}

impl World {
    /// Create a new `World` instance that can draw a moving box.
    pub fn new(width: u32, height: u32) -> Self {
        let size = width.checked_mul(height).expect("too big") as usize;
        Self {
            elements: vec![Substance::air(); size],
            scrach_elements: vec![Substance::air(); size],
            width: width,
            height: height,
            drop_sand: false,
            drop_water: false,
        }
    }

    pub fn drop(&mut self, kind: SubstanceKind, x: u32, y: u32) {
        let index = y * self.width + x;

        match kind {
            SubstanceKind::SAND => {
                self.elements[index as usize] = Substance::sand();
            }
            SubstanceKind::WATER => {
                self.elements[index as usize] = Substance::water();
            }
            SubstanceKind::STONE => {
                self.elements[index as usize] = Substance::stone();
            }
            _ => {
                return;
            }
        }
        self.scrach_elements = self.elements.clone();
    }
}

impl Worldable for World {
    fn setup(&mut self) {
        let size = (self.width.checked_mul(self.height).expect("too big") - self.width) as usize;
        let stone = vec![Substance::stone(); self.width as usize];
        self.elements.splice(size.., stone);

        let start_index =
            (self.width.checked_mul(50).expect("too big") + self.width / 2 - 5) as usize;
        let end_index = start_index + 5;
        self.elements
            .splice(start_index..end_index, vec![Substance::stone(); 9]);

        self.scrach_elements = self.elements.clone();
    }

    fn key_press(&mut self, input: &WinitInputHelper) {
        if input.key_pressed(VirtualKeyCode::A) {
            self.drop_sand = true;
        } else if input.key_pressed(VirtualKeyCode::S) {
            self.drop_water = true;
        } else {
            self.drop_sand = false;
            self.drop_water = false;
        }
    }

    fn mouse_press(&mut self, at: (u32, u32)) {
        self.drop(SubstanceKind::STONE, at.0, at.1);
    }

    fn update(&mut self) {
        if self.drop_sand {
            self.drop(SubstanceKind::SAND, self.width / 2, 0);
        }

        if self.drop_water {
            self.drop(SubstanceKind::WATER, self.width / 2, 0);
        }

        let mut iterable = self.elements.clone();
        for (i, element) in iterable.iter_mut().enumerate() {
            // Current element is not updatable, no need to do anything
            if !element.is_updatable {
                continue;
            }

            // Current element is updatable and element below it is not stationary
            // so we can swap them
            let index_of_element_below =
                min(i as u32 + self.width, self.width * self.height - 1) as usize;
            let element_below = &self.elements[index_of_element_below];
            if !element_below.is_stationary || element.density > element_below.density {
                self.scrach_elements.swap(i, index_of_element_below);
                continue;
            }

            // Current element is not stationary and element below it is stationary
            let factor: i32 = if random_bool() { -1 } else { 1 };
            let index_of_below_random_side = min(
                i as u32 + (-factor + self.width as i32) as u32,
                self.width * self.height - 1,
            ) as usize;
            let element_below_to_random_side = &self.elements[index_of_below_random_side];

            if element.kind == SubstanceKind::SAND {
                if !element_below_to_random_side.is_stationary {
                    self.scrach_elements.swap(i, index_of_below_random_side);
                    continue;
                }
            }

            if element.kind == SubstanceKind::WATER {
                if !element_below_to_random_side.is_stationary {
                    self.scrach_elements.swap(i, index_of_below_random_side);
                    continue;
                }

                // Swap with element to random side
                let index_of_random_side =
                    min((i as i32) - factor, (self.width * self.height - 1) as i32) as usize;

                let element_on_random_side = &self.elements[index_of_random_side];
                if element_on_random_side.is_stationary {
                    continue;
                }

                self.scrach_elements.swap(i, index_of_random_side);
                continue;
            }
        }
        self.elements = self.scrach_elements.clone();
    }

    /// Draw the `World` state to the frame buffer.
    ///
    /// Assumes the default texture format: `wgpu::TextureFormat::Rgba8UnormSrgb`
    fn draw(&self, frame: &mut [u8]) {
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let color = self.elements[i].color;

            pixel.copy_from_slice(&color);
        }

        // for (element, pixel) in self.elements.iter().zip(frame.chunks_exact_mut(4)) {
        //     element.draw(pixel);
        // }
    }
}
