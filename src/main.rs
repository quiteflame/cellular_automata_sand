pub mod air;
pub mod app;
pub mod element;
pub mod sand;
pub mod stone;
pub mod substance;
pub mod world;
pub mod worldable;

const WIDTH: u32 = 320;
const HEIGHT: u32 = 200;

use app::App;
use world::World;

fn main() {
    env_logger::init();
    let app = App::new(WIDTH, HEIGHT, Box::new(World::new(WIDTH, HEIGHT)));
    app.run();
}
