pub mod ring_queue;

#[derive(Debug)]
#[allow(dead_code)]
pub enum ContainerError {
    NoSpaceError,
    NoItemsError,
}
