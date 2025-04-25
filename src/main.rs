pub mod buddies;
pub mod renderer;
pub mod config;

use config::Config;
use glfw::{Action, Context, Key};

fn main() {
    //@TODO: impl desktop pet
    println!("hello meow");
    let config = Config::init().expect("fuck");
    println!("{:#?}", config);
}