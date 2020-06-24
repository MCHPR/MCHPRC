use config::Config;
use glfw::{Action, Context, Glfw, Key, SwapInterval, WindowHint};
use std::convert::TryFrom;
use std::sync::mpsc::Receiver;

pub struct Window {
    pub width: u32,
    pub height: u32,
    pub title: String,
    pub glfw: Glfw,
    pub glfw_window: glfw::Window,
    pub glfw_events: Receiver<(f64, glfw::WindowEvent)>,
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
            .expect("'window_x' unset in Config.toml");
        let y = config
            .get_int("window_y")
            .expect("'window_y' unset in Config.toml");

        let x = u32::try_from(x).expect("Invalid 'window_x' set");
        let y = u32::try_from(y).expect("Invalid 'window_y' set");

        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

        glfw.window_hint(WindowHint::ContextVersion(3, 2));
        glfw.window_hint(WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
        glfw.window_hint(WindowHint::OpenGlForwardCompat(true));

        let (mut window, events) = glfw
            .create_window(x, y, &title, glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window.");

        glfw.make_context_current(Some(&window));
        // Uncomment to disable VSync
        //glfw.set_swap_interval(SwapInterval::None);

        window.set_framebuffer_size_polling(true);
        window.set_key_polling(true);
        window.make_current();

        return Window {
            width: x,
            height: y,
            title,
            glfw,
            glfw_window: window,
            glfw_events: events,
        };
    }

    pub fn update(&mut self) {
        self.glfw_window.swap_buffers();
        self.glfw.poll_events();

        let messages: Vec<_> = glfw::flush_messages(&self.glfw_events).collect();

        for (_, event) in messages {
            self.handle_window_event(event);
        }
    }

    pub fn handle_window_event(&mut self, event: glfw::WindowEvent) {
        match event {
            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                self.glfw_window.set_should_close(true);
            }
            _ => {}
        }
    }
}
