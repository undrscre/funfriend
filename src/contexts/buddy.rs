use crate::{
    WindowContext, buddies::{Buddy, DialogType}, config::ConfigType, renderer::{buddy::BuddyRenderer, ease::Ease},
};
use cgmath::{MetricSpace, Vector2, Zero};
use glfw::{Action, Context, Key};
use rand::{Rng, prelude::IndexedRandom};

pub const STAY_STILL_AFTER_HELD: f64 = 1.0;
pub const WANDER_TIMER: f64 = 4.;
pub const CHATTER_TIMER: f64 = 3.0;
pub const FOLLOW_DIST: f64 = 120.;

enum Behavior {
    Wander,
    Follow,
    // Stay
}

pub struct BuddyContext {
    pub buddy: Box<dyn Buddy>, //@TODO impl this as just Box<Dyn Buddy>
    pub renderer: BuddyRenderer,
    pub config: ConfigType,
    pub window: WindowContext,

    pub chatter_timer: f64,
    pub chatter_index: usize,
    pub chatter_array: Vec<&'static str>,

    pub held: bool,
    pub held_at: Vector2<f64>,
    pub started_holding_at: Vector2<f64>,
    pub waiting_for_stable_pos: bool,

    pub held_timer: f64,
    pub static_pos: Vector2<f64>,
    pub wander_timer: f64,

    pub easing_from: Vector2<f64>,
    pub easing_to: Vector2<f64>,
    pub easing_dur: f64,
    pub easing_t: f64,
}

impl BuddyContext {
    pub fn new(buddy: Box<dyn Buddy>, renderer: BuddyRenderer, config: ConfigType, window: WindowContext) -> Self {
        Self {
            buddy,
            renderer,
            config,
            window,
            chatter_timer: 1.0,
            chatter_index: 0,
            chatter_array: vec![""],
            held: false,
            held_at: Vector2::zero(),
            started_holding_at: Vector2::zero(),
            waiting_for_stable_pos: false,
            held_timer: 0.0,
            static_pos: Vector2::zero(),
            wander_timer: WANDER_TIMER,
            easing_from: Vector2::zero(),
            easing_to: Vector2::zero(),
            easing_dur: 0.,
            easing_t: 0.,
        }
    }

    pub fn init(&mut self) {
        self.window
            .handle
            .set_title(format!("??__{}__??", self.buddy.name()).as_str());

        self.window.handle.make_current();

        let mut rng = rand::rng();
        self.window.glfw.with_primary_monitor(|_, monitor| {
            let monitor_pos = monitor.as_ref().unwrap().get_pos();
            let mode = monitor.as_ref().unwrap().get_video_mode().unwrap();
        
            let random_pos = (
                monitor_pos.0 as f64 + (mode.width as f64 * rng.random::<f64>()),
                monitor_pos.1 as f64 + (mode.height as f64 * rng.random::<f64>()),
            );
        
            self.window.handle.set_pos(random_pos.0 as i32, random_pos.1 as i32);
            self.static_pos = Vector2::new(random_pos.0, random_pos.1);
            self.easing_to = Vector2::new(random_pos.0, random_pos.1);
            self.easing_from = Vector2::new(random_pos.0, random_pos.1);
            // self.goto(Vector2::new(random_pos.0, random_pos.1), 0.0, true);
        });        
    }

    pub fn update(&mut self, dt: f64) {
        self.chatter_timer -= dt;
        if self.chatter_timer <= 0. {
            self.chatter_timer = self.chatter_timer - dt;
            if let Some(text) = self.chatter_array.get(self.chatter_index) {
                if !text.is_empty() {
                    self.say(text);
                }
            }
            self.chatter_index += 1;
        }

        self.update_pos(dt);
        self.render(dt);
        self.window.handle.swap_buffers();
    }
    
    fn render(&mut self, dt: f64) {
        let size = (self.config.friend_size as f64 * 1.3).floor() as i32;
        self.window.handle.make_current();
        self.renderer.render(dt, self.config, size, self.window.glfw.get_time() as f32);
    }
    
    pub fn handle_event(&mut self, event: glfw::WindowEvent) {
        match event {
            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                self.window.handle.set_should_close(true);
            }
            glfw::WindowEvent::MouseButton(glfw::MouseButtonLeft, act, _) => {
                match act {
                    Action::Press => {
                        let cursor_pos = self.window.handle.get_cursor_pos();
                        self.buddy.talk_sound();
                        self.easing_dur = 0.;
                        self.held = true;
                        self.held_at = Vector2::new(cursor_pos.0, cursor_pos.1);
                        if self.held_timer <= 0. {
                            let pos = self.window.handle.get_pos();
                            self.started_holding_at = Vector2::new(pos.0 as f64, pos.1 as f64);
                        }
                        self.held_timer = STAY_STILL_AFTER_HELD;
                        self.window
                            .handle
                            .set_cursor(Some(glfw::Cursor::standard(glfw::StandardCursor::Hand)));
                    }
                    Action::Release => {
                        self.held = false;
                        self.window
                            .handle
                            .set_cursor(Some(glfw::Cursor::standard(glfw::StandardCursor::Arrow)));
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
    
    pub fn say_random_sequence(&mut self, text_options: Vec<Vec<&'static str>>) {
        if !text_options.is_empty() {
            let mut rng = rand::rng();
            if let Some(selected_sequence) = text_options.choose(&mut rng) {
                self.chatter_array = selected_sequence.to_vec();
                self.chatter_timer = 0.0;
                self.chatter_index = 0;
            }
        }
    }

    pub fn say(&mut self, text: &'static str) {
        println!("{}", text);
    }

    fn is_moving(&self) -> bool {
        self.easing_dur != 0. && self.easing_t <= self.easing_dur
    }

    fn is_speaking(&self) -> bool {
        !self.chatter_array.is_empty() && (self.chatter_index as usize) < self.chatter_array.len()
    }

    fn get_behavior(&mut self) -> Behavior {
        if self.is_speaking() {
            Behavior::Follow
        } else {
            Behavior::Wander
        }
    }

    fn goto(&mut self, pos: Vector2<f64>, dur: f64, set_as_static: bool) {
        let window_pos = self.window.handle.get_pos();
        self.easing_from = Vector2::new(window_pos.0 as f64, window_pos.1 as f64);
        self.easing_to = pos;
        self.easing_dur = dur;
        self.easing_t = 0.0;

        if set_as_static {
            self.static_pos = pos;
        }
    }

    fn update_wander(&mut self, dt: f64) {
        if self.is_moving() {
            self.easing_t += dt;
            let val = Ease::in_out_sine(self.easing_t / self.easing_dur);
            let pos = self.easing_from * (1.0 - val) + self.easing_to * val;
            self.window.handle.set_pos(pos.x as i32, pos.y as i32); 
            self.wander_timer = WANDER_TIMER;
        } else {
            match self.get_behavior() {
                Behavior::Wander => {
                    self.wander_timer -= dt;
                    if self.wander_timer <= 0. {
                        let mut rng = rand::rng();
                        let rand = Vector2::new(
                            rng.random_range(-150.0..150.0),
                            rng.random_range(-150.0..150.0),
                        );
                        self.goto(self.static_pos + rand, 4.0, false);
                    }
                }
                Behavior::Follow => {
                    if !self.is_moving() {
                        let cursor_pos = self.window.handle.get_cursor_pos();
                        let window_pos = self.window.handle.get_pos();

                        let x_dist = cursor_pos.0;
                        let y_dist = cursor_pos.1;

                        let mut x_target = window_pos.0 as f64;
                        let mut y_target = window_pos.1 as f64;

                        if x_dist.abs() > FOLLOW_DIST {
                            x_target = (window_pos.0 as f64 + x_dist) - (FOLLOW_DIST * x_dist.signum())
                        }
                        if y_dist.abs() > FOLLOW_DIST {
                            y_target = (window_pos.1 as f64 + y_dist) - (FOLLOW_DIST * y_dist.signum())
                        }
                        self.goto(Vector2::new(x_target, y_target), 0.5, true);
                    }
                }
            }
        }
    }

    fn update_pos(&mut self, dt: f64) {
        if self.held {
            let window_pos = self.window.handle.get_pos();
            let cursor_pos = self.window.handle.get_cursor_pos();
            self.static_pos = Vector2::new(window_pos.0 as f64, window_pos.1 as f64) - self.held_at + Vector2::new(cursor_pos.0, cursor_pos.1);
            self.window.handle.set_pos(self.static_pos.x as i32, self.static_pos.y as i32);
        } else {
            self.held_timer -= dt;
            if self.held_timer <= 0. {
                self.update_wander(dt);
                if self.waiting_for_stable_pos {
                    self.waiting_for_stable_pos = false;
                    let stable_pos_dist = self.static_pos.distance(self.started_holding_at);
                    if !self.is_speaking() {
                        if stable_pos_dist > 50. {
                            self.say_random_sequence(self.buddy.dialog(DialogType::Moved));
                        } else {
                            self.say_random_sequence(self.buddy.dialog(DialogType::Touched));
                        }
                    }
                }
            } else {
                self.waiting_for_stable_pos = true;
            }
        }
    }
}
