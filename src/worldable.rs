use winit_input_helper::WinitInputHelper;

pub trait Worldable {
    fn setup(&mut self);
    fn key_press(&mut self, input: &WinitInputHelper);
    fn mouse_press(&mut self, at: (u32, u32));
    fn update(&mut self);
    fn draw(&self, frame: &mut [u8]);
}
