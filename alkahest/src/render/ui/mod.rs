mod root;
pub use root::UIRoot;
mod panel;
pub use panel::UIPanel;

use super::Batch;
use ultraviolet::Mat4;

pub trait UIElement {
    fn get_data_for_buffer(&self, tranform: Mat4) -> Batch;
}
