pub mod buddies;
pub mod config;
pub mod renderer;
pub mod contexts;

use glfw::fail_on_errors;
use crate::config::Config;
use crate::contexts::{buddy::BuddyContext, window::WindowContext};
use crate::renderer::buddy::BuddyRenderer;

use std::sync::{Arc, Mutex};

fn read_file(file: &str) -> String {
    std::fs::read_to_string(file).unwrap()
}

fn main() {
    println!("hello meow");
    let glfw = Arc::new(Mutex::new(glfw::init(glfw::fail_on_errors!()).unwrap()));
    let config = Config::init()
        .expect("config fail")
        .config;

    let vert = read_file("src/shaders/default.vert");
    let frag = read_file("src/shaders/funfriend.frag");

    let buddy = buddies::retrieve_buddy(&config.friend_type);
    let window = WindowContext::new(glfw, config.friend_size, config.friend_size, "hello bro", true);
    let renderer = BuddyRenderer::new(buddy.textures(), vert.as_str(), frag.as_str())
        .expect("renderer init fail");

    let mut context = BuddyContext::new(buddy, renderer, config, window);
    context.init();

    let asdfhjkl = context.window.glfw.clone();
    {
        let mut context_glfw = asdfhjkl.lock().unwrap();
        let mut time = context_glfw.get_time();
        while !context.window.handle.should_close() {
            let dt = context_glfw.get_time() - time;
            time = context_glfw.get_time();

            context.window.update(dt);
            context_glfw.poll_events();
            context.update(dt, time);

            let events: Vec<_> = glfw::flush_messages(&context.window.events).collect();
            for (_, event) in events {
                context.handle_event(event);
            }
        }
    }
}
