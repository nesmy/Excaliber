use window::run;

pub mod window;

pub fn start() {
    pollster::block_on(run());
}