use alkahest::{info, trace};
use alkahest::util::input;

struct Sandbox;
impl Sandbox {
    fn init() {
        trace!("In Sandbox::init()!");
    }

    fn update() {
        if input::is_key_down(input::Key::Space) {
            info!("Pressed space!");
        }
    }

    fn cleanup() {
        trace!("In Sandbox::cleanup()!");
    }
}

fn main() {
    alkahest::run(Sandbox::init, Sandbox::update, Sandbox::cleanup);
}
