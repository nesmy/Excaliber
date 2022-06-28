use Excaliber::entry::run;

pub fn main() {
    pollster::block_on(run());
}