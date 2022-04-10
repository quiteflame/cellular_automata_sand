pub trait Element {
    fn x(&self) -> i32;
    fn y(&self) -> i32;
    fn color(&self) -> [u8; 4];
    fn draw(&self, pixel: &mut [u8]);
    fn update(&mut self);
    fn is_stationary(&self) -> bool;
}
