use std::fmt::Display;
use winit::event::VirtualKeyCode;
use winit_input_helper::WinitInputHelper;

use crate::{element::Element, worldable::Worldable};

pub struct World {
    pub width: usize,
    pub height: usize,
    matrix: Vec<Element>,
    scrap_matrix: Vec<Element>,
}

impl Display for World {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let combined = self.description(self.matrix.clone());
        write!(f, "{}", combined)
    }
}

impl World {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            matrix: Vec::<Element>::new(),
            scrap_matrix: Vec::<Element>::new(),
        }
    }

    fn description(&self, elements: Vec<Element>) -> String {
        let mut combined = String::new();

        for (index, element) in elements.iter().enumerate() {
            if index % self.width == 0 {
                combined.push_str("\n");
            }
            combined.push_str(&element.to_string());
        }

        combined
    }

    pub fn add(&mut self, element: Element, index: usize) {
        self.matrix[index] = element.clone();
        self.scrap_matrix[index] = element;
    }

    pub fn element(&self, index: usize) -> &Element {
        &self.scrap_matrix[index]
    }

    pub fn index(&self, x: usize, y: usize) -> usize {
        x + y * self.width
    }

    pub fn switch(&mut self, index1: usize, index2: usize) {
        self.scrap_matrix.swap(index1, index2);
    }

    pub fn within_bounds(&self, x: i32) -> bool {
        x >= 0 && x < self.width as i32
    }
}

impl Worldable for World {
    fn setup(&mut self) {
        let size = self.width * self.height;
        for _ in 0..size {
            self.matrix.push(Element::air());
        }

        let start_index = self.width * (self.height - 1);
        for index in start_index..size {
            self.matrix[index] = Element::stone();
        }

        self.scrap_matrix = self.matrix.clone();
    }

    fn key_press(&mut self, input: &WinitInputHelper) {
        if input.key_pressed(VirtualKeyCode::A) {
            self.add(Element::sand(), self.index(self.width / 2, 0));
        } else if input.key_pressed(VirtualKeyCode::S) {
            self.add(Element::water(), self.index(self.width / 2, 0));
        }
    }

    fn mouse_press(&mut self, at: (u32, u32)) {
        self.add(Element::stone(), self.index(at.0 as usize, at.1 as usize));
    }

    fn update(&mut self) {
        let mut elements = self.matrix.clone();

        for (index, element) in elements.iter_mut().enumerate().rev() {
            element.step(self, index);
        }

        self.matrix = self.scrap_matrix.clone();
    }

    /// Draw the `World` state to the frame buffer.
    ///
    /// Assumes the default texture format: `wgpu::TextureFormat::Rgba8UnormSrgb`
    fn draw(&self, frame: &mut [u8]) {
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let color = self.matrix[i].color;

            pixel.copy_from_slice(&color);
        }

        // for (element, pixel) in self.elements.iter().zip(frame.chunks_exact_mut(4)) {
        //     element.draw(pixel);
        // }
    }
}
