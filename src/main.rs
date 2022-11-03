use alkahest::{info, trace};
use alkahest::util::input;

mod util;

struct Sandbox {
    pub project: Option<util::project::ProjectContext>,
}

impl alkahest::Application for Sandbox {
    fn init(&mut self) {
        self.project = util::project::init(&String::from("/home/anthony/.alkahest/projects/main")).ok();
    }

    fn update(&mut self) {
        if input::is_key_down(input::Key::Space) {
            info!("Pressed space!");
        }
    }

    fn cleanup(&mut self) {
        trace!("In Sandbox::cleanup()!");
    }
}

fn main() {
    let mut s = Sandbox { project: None };
    alkahest::run(&mut s);
}
