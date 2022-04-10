use crate::element::Element;

pub struct Stone {
    x: i32,
    y: i32,
    rgba: [u8; 4],
}

impl Element for Stone {
    fn x(&self) -> i32 {
        self.x
    }

    fn y(&self) -> i32 {
        self.y
    }

    fn color(&self) -> [u8; 4] {
        self.rgba
    }

    fn draw(&self, pixel: &mut [u8]) {
        pixel.copy_from_slice(&self.color());
    }

    fn update(&mut self) {}

    fn is_stationary(&self) -> bool {
        true
    }
}

impl Stone {
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            x,
            y,
            rgba: [0x7f, 0x7f, 0x7f, 0xff],
        }
    }
}
