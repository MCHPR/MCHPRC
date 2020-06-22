use crate::window::Window;
use crate::render::Renderer;
use config::Config;

pub struct Client {
    config: Config,
    window: Window,
    renderer: Renderer
}

impl Client {
    pub fn run() {
        let mut config = config::Config::default();

        config
            .merge(config::File::with_name("Config"))
            .expect("Unable to load Config.toml");

        let mut window = Window::init(&config);

        let renderer = Renderer::init(&mut window.glfw_window);

        let mut client = Client {
            config,
            window,
            renderer
        };

        while !client.window.glfw_window.should_close() {

            client.renderer.update();
            client.window.update();

        }
    }
}
