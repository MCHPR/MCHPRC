mod client;
mod player;
mod render;
mod render_camera;
mod spatial;
mod window;

use client::Client;

fn main() {
    Client::run();
}
