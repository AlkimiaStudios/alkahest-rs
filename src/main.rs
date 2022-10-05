use alkahest::trace;

struct Sandbox;
impl Sandbox {
    fn init() {
        trace!("In Sandbox::init()!");
    }

    fn update() {
    }

    fn cleanup() {
        trace!("In Sandbox::cleanup()!");
    }
}

fn main() {
    alkahest::run(Sandbox::init, Sandbox::update, Sandbox::cleanup);
}
