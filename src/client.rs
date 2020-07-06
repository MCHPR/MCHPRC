use nalgebra::Vector3;
use crate::render::Renderer;
use crate::window::Window;
use crate::player::Player;
use config::Config;

#[allow(dead_code)]
pub struct Client {
    config: Config,
    window: Window,
    renderer: Renderer,
    player: Player,
}

impl Client {
    pub fn run() {
        let mut config = config::Config::default();

        config
            .merge(config::File::with_name("Config"))
            .expect("Unable to load Config.toml");

        let mut window = Window::init(&config);

        let renderer = Renderer::init(&mut window.glfw_window);

        let mut player = Player::new();
        player.spatial.set_translation(&Vector3::new(0.0, 0.0, 3.0));

        let mut client = Client {
            config,
            window,
            renderer,
            player,
        };

        while !client.window.glfw_window.should_close() {
            let mut control_vector = Vector3::new(0.0, 0.0, 0.0);
            if client.window.control_state.forward {
                control_vector[2] -= 1.0;
            }
            if client.window.control_state.back {
                control_vector[2] += 1.0;
            }
            if client.window.control_state.left {
                control_vector[0] -= 1.0;
            }
            if client.window.control_state.right {
                control_vector[0] += 1.0;
            }
            if client.window.control_state.up {
                control_vector[1] += 1.0;
            }
            if client.window.control_state.down {
                control_vector[1] -= 1.0;
            }

            client.player.set_control_vector(&control_vector);
            client.player.update();

            let mut camera_spatial = client.renderer.camera
                .borrow_spatial_mut();
            camera_spatial.set_translation(client.player.spatial
                .get_translation());
            camera_spatial.set_rotation(client.player.spatial
                .get_rotation());

            client.renderer.update();
            client.window.update();
        }
    }
}
