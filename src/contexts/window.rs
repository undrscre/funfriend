use std::sync::{Arc, Mutex};
use glfw::Context;

pub struct WindowContext {
    pub glfw: Arc<Mutex<glfw::Glfw>>,
    pub handle: glfw::PWindow,
    pub events: glfw::GlfwReceiver<(f64, glfw::WindowEvent)>,
}

impl WindowContext {
    pub fn new(glfw: Arc<Mutex<glfw::Glfw>>, width: u32, height: u32, title: &str, transparent: bool) -> Self {
        let real_glfw = glfw.clone();
        let mut glfw = glfw.lock().unwrap();
        {
            glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
            glfw.window_hint(glfw::WindowHint::Decorated(false));
            glfw.window_hint(glfw::WindowHint::Resizable(false));
            glfw.window_hint(glfw::WindowHint::Focused(false));
            glfw.window_hint(glfw::WindowHint::FocusOnShow(false));
            glfw.window_hint(glfw::WindowHint::Floating(true));
            if transparent {
                glfw.window_hint(glfw::WindowHint::TransparentFramebuffer(true));
            }
            glfw.window_hint(glfw::WindowHint::AlphaBits(Some(8)));

            let (mut window, events) = glfw
                .create_window(width, height, title, glfw::WindowMode::Windowed)
                .expect("failed to create GLFW window");

            window.set_all_polling(true);
            gl::load_with(|s| window.get_proc_address(s) as *const _);
            glfw.set_swap_interval(glfw::SwapInterval::None);
            Self {
                glfw: real_glfw,
                handle: window,
                events,
            }
        }
    }

    pub fn update(&mut self, _dt: f64) {
        self.handle.make_current();
        self.handle.swap_buffers();
    }

    pub fn close(&mut self) {
        self.handle.set_should_close(true);
    }
}
