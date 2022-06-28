mod window;
mod texture;

pub mod entry;

use entry::run;

pub fn start() {
    pollster::block_on(run());
}