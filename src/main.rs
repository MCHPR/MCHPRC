mod client;
mod render;
mod spatial;
mod render_camera;
mod window;
mod player;

use client::Client;

fn main() {
    Client::run();
}
