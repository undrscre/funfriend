pub mod buddies;
pub mod renderer;
pub mod config;

use renderer::window;
use config::Config;
use glfw::{Action, Context, Key};

fn main() {
    //@TODO: impl desktop pet
    println!("hello meow");
    println!("{:#?}", Config::retrieve_config_path().unwrap().to_str());
}