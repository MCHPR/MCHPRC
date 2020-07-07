use config::Config;
use glfw::{Action, Context, Glfw, Key, SwapInterval, WindowHint, CursorMode};
use std::convert::TryFrom;
use std::sync::mpsc::Receiver;

pub struct Window {
    pub width: u32,
    pub height: u32,
    pub title: String,
    pub glfw: Glfw,
    pub glfw_window: glfw::Window,
    pub glfw_events: Receiver<(f64, glfw::WindowEvent)>,
    pub control_state: ControlState,
}

pub struct ControlState {
    pub forward: bool,
    pub back: bool,
    pub left: bool,
    pub right: bool,
    pub up: bool,
    pub down: bool,
    pub pitch: f32,
    pub yaw: f32,
    pub last_cursor_x: f64,
    pub last_cursor_y: f64,
}

impl Window {
    pub fn init(config: &Config) -> Window {
        let title = format!(
            "{} - Version {}",
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION")
        );

        let x = config
            .get_int("window_x")
            .expect(unset_setting("window_x").as_ref());
        let y = config
            .get_int("window_y")
            .expect(unset_setting("window_y").as_ref());

        let x = u32::try_from(x).expect(invalid_setting("window_x").as_ref());
        let y = u32::try_from(y).expect(invalid_setting("window_y").as_ref());

        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

        glfw.window_hint(WindowHint::ContextVersion(3, 2));
        glfw.window_hint(WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
        glfw.window_hint(WindowHint::OpenGlForwardCompat(true));

        let (mut window, events) = glfw
            .create_window(x, y, &title, glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window.");

        glfw.make_context_current(Some(&window));

        if !config
            .get_bool("vsync")
            .expect(unset_setting("vsync").as_ref())
        {
            glfw.set_swap_interval(SwapInterval::None);
        }

        window.set_framebuffer_size_polling(true);
        window.set_key_polling(true);
        window.make_current();
        window.set_cursor_mode(CursorMode::Disabled);

        let control_state = ControlState::new();

        return Window {
            width: x,
            height: y,
            title,
            glfw,
            glfw_window: window,
            glfw_events: events,
            control_state,
        };
    }

    pub fn update(&mut self) {
        self.glfw_window.swap_buffers();
        self.glfw.poll_events();

        let messages: Vec<_> = glfw::flush_messages(&self.glfw_events).collect();

        for (_, event) in messages {
            self.handle_window_event(event);
        }

        // Update our control state based on mouse position.

        // GLFW tracks the cursor position as a pair of doubles in an
        // unbounded space while the cursor is disabled, we have to
        // calculate the mouse movement deltas outselves.
        let cursor_pos = self.glfw_window.get_cursor_pos();
        let cursor_delta_x = cursor_pos.0 - self.control_state
            .last_cursor_x;
        self.control_state.last_cursor_x = cursor_pos.0;
        let cursor_delta_y = cursor_pos.1 - self.control_state
            .last_cursor_y;
        self.control_state.last_cursor_y = cursor_pos.1;
        
        self.control_state.pitch -= (cursor_delta_y as f32) * 0.03;
        self.control_state.yaw -= (cursor_delta_x as f32) * 0.03;

        // Bound the pitch to 90 degrees in both directions.
        if (self.control_state.pitch < -90.0) {
            self.control_state.pitch = -90.0;
        }
        else if (self.control_state.pitch > 90.0) {
            self.control_state.pitch = 90.0;
        }

        // Wrap the yaw to values between 0 and 360 degrees.
        while (self.control_state.yaw < 0.0 || 
                self.control_state.yaw > 360.0) {
            self.control_state.yaw -= 360.0 
                * self.control_state.yaw.signum();
        }
    }

    pub fn handle_window_event(&mut self, event: glfw::WindowEvent) {
        match event {
            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                self.glfw_window.set_should_close(true);
            }
            glfw::WindowEvent::Key(Key::W, _, Action::Press, _) => {
                self.control_state.forward = true;
            }
            glfw::WindowEvent::Key(Key::W, _, Action::Release, _) => {
                self.control_state.forward = false;
            }
            glfw::WindowEvent::Key(Key::S, _, Action::Press, _) => {
                self.control_state.back = true;
            }
            glfw::WindowEvent::Key(Key::S, _, Action::Release, _) => {
                self.control_state.back = false;
            }
            glfw::WindowEvent::Key(Key::A, _, Action::Press, _) => {
                self.control_state.left = true;
            }
            glfw::WindowEvent::Key(Key::A, _, Action::Release, _) => {
                self.control_state.left = false;
            }
            glfw::WindowEvent::Key(Key::D, _, Action::Press, _) => {
                self.control_state.right = true;
            }
            glfw::WindowEvent::Key(Key::D, _, Action::Release, _) => {
                self.control_state.right = false;
            }
            glfw::WindowEvent::Key(Key::Space, _, Action::Press, _) => {
                self.control_state.up = true;
            }
            glfw::WindowEvent::Key(Key::Space, _, Action::Release, _) => {
                self.control_state.up = false;
            }
            glfw::WindowEvent::Key(Key::LeftShift, _, Action::Press, _) => {
                self.control_state.down = true;
            }
            glfw::WindowEvent::Key(Key::LeftShift, _, Action::Release, _) => {
                self.control_state.down = false;
            }

            _ => {}
        }
    }
}

pub fn invalid_setting(setting: &str) -> String {
    format!("Invalid '{}' set in Config.toml", setting)
}
pub fn unset_setting(setting: &str) -> String {
    format!("'{}' unset in Config.toml", setting)
}

impl ControlState {
    fn new() -> ControlState {
        ControlState {
            forward: false,
            back: false,
            left: false,
            right: false,
            up: false,
            down: false,
            pitch: 0.0,
            yaw: 0.0,
            last_cursor_x: 0.0,
            last_cursor_y: 0.0,
        }
    }
}
