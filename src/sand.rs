use rand::Rng;

use crate::element::Element;

pub struct Sand {
    x: i32,
    y: i32,
    rgba: [u8; 4],
    alive: bool,
}

impl Element for Sand {
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
        let color = if self.alive {
            self.color()
        } else {
            [0xff, 0xff, 0xe8, 0xff]
        };
        pixel.copy_from_slice(&color);
    }

    fn update(&mut self) {
        let mut rng = rand::thread_rng();

        self.alive = if rng.gen_range(0..10 as i32) < 5 {
            true
        } else {
            false
        };
    }

    fn is_stationary(&self) -> bool {
        false
    }
}

impl Sand {
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            x,
            y,
            rgba: [0x5e, 0x48, 0xe8, 0xff],
            alive: true,
        }
    }
}
