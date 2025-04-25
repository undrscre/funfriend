pub mod buddies;
pub mod config;
pub mod renderer;
pub mod contexts;

use crate::config::Config;
use renderer::buddy::BuddyRenderer;
use crate::contexts::{buddy::BuddyContext, window::WindowContext};

fn main() {
    //@TODO: impl desktop pet
    println!("hello meow");
    let config = Config::init().expect("couldn't initialize configuration").config;
    let buddy = buddies::retrieve_buddy(&config.friend_type);
    let window = WindowContext::new(config.friend_size, config.friend_size, "Hello bro", false);
    let mut context = BuddyContext::new(buddy, BuddyRenderer {}, config, window);

    context.init();

    let mut last_t = context.window.glfw.get_time();
    while !context.window.handle.should_close() {
        let dt = context.window.glfw.get_time() - last_t;
        last_t = context.window.glfw.get_time();

        context.window.update(dt);
        context.window.glfw.poll_events();
        
        context.update(dt);
        let events: Vec<_> = glfw::flush_messages(&context.window.events).collect();
        for (_, event) in events {
            context.handle_event(event);
        }
    }
}
