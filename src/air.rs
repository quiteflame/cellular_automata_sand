use crate::element::Element;

pub struct Air {
    x: i32,
    y: i32,
    rgba: [u8; 4],
}

impl Element for Air {
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
        false
    }
}

impl Air {
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            x,
            y,
            rgba: [0x0, 0x0, 0x0, 0xff],
        }
    }
}
