mod client;
mod render;
mod render_camera;
mod spatial;
mod window;
mod player;

use client::Client;

fn main() {
    Client::run();
}
