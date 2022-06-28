mod state;
mod texture;
mod camera;
mod instance;

pub mod entry;

use entry::run;

pub fn start() {
    pollster::block_on(run());
}