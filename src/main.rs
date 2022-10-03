struct Sandbox;
impl Sandbox {
    fn init() {
        println!("In Sandbox::init()!");
    }

    fn update() {
    }

    fn cleanup() {
        println!("In Sandbox::cleanup()!");
    }
}

fn main() {
    alkahest::run(Sandbox::init, Sandbox::update, Sandbox::cleanup);
}
