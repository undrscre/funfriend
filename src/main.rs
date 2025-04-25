pub mod buddies;
pub mod renderer;
pub mod config;

use crate::{
    config::Config,
    renderer::window::Window
};

use glfw::{Action, Context, Key};

fn main() {
    //@TODO: impl desktop pet
    println!("hello meow");
    let config = Config::init().expect("fuck").config;
    let mut window = Window::new(config.friend_size, config.friend_size, "test");

    while !window.handle.should_close() {
        window.handle.swap_buffers();
        window.glfw.poll_events();
        for (_, event) in glfw::flush_messages(&window.events) {
            println!("{:?}", event);
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    window.handle.set_should_close(true)
                },
                _ => {},
            }
        }
    }
}