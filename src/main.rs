pub mod app;
pub mod element;
pub mod world;
pub mod worldable;

const WIDTH: u32 = 320;
const HEIGHT: u32 = 200;

use app::App;
use world::World;

fn main() {
    env_logger::init();
    let app = App::new(
        WIDTH,
        HEIGHT,
        Box::new(World::new(WIDTH as usize, HEIGHT as usize)),
    );
    app.run();
}
