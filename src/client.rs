extern crate glfw;

use glfw::{Action, Context, Key};
use std::convert::TryFrom;
use self::glfw::WindowHint;

pub struct Client {
    config: config::Config,
    window_x: u32,
    window_y: u32,
}

impl Client {
    pub fn run() {
        let mut config = config::Config::default();

        config
            .merge(config::File::with_name("Config"))
            .expect("Unable to load Config.toml");

        let x = config
            .get_int("window_x")
            .expect("'window_x' unset in Config.toml");
        let y = config
            .get_int("window_y")
            .expect("'window_y' unset in Config.toml");

        let x = u32::try_from(x).expect("Invalid 'window_x' set");
        let y = u32::try_from(y).expect("Invalid 'window_y' set");

        let mut client = Client {
            config,
            window_x: x,
            window_y: y,
        };

        client.start_glfw();
    }

    fn start_glfw(&mut self) {
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

        let title = format!(
            "{} - Version {}",
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION")
        );

        let (mut window, events) = glfw
            .create_window(
                self.window_x,
                self.window_y,
                &title,
                glfw::WindowMode::Windowed,
            )
            .expect("Failed to create GLFW window.");

        window.set_key_polling(true);
        window.make_current();

        while !window.should_close() {
            glfw.poll_events();
            for (_, event) in glfw::flush_messages(&events) {
                self.handle_window_event(&mut window, event);
            }
        }
    }

    fn handle_window_event(&mut self, window: &mut glfw::Window, event: glfw::WindowEvent) {
        match event {
            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                window.set_should_close(true)
            }
            _ => {}
        }
    }
}
