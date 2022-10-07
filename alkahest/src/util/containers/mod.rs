mod ring_buffer;

pub use ring_buffer::RingBuffer;

/// Errors used by container structs
#[derive(Debug)]
#[allow(dead_code)]
pub enum ContainerError {
    /// Error returned when trying to allocate space in a full container
    NoSpaceError,
    /// Error returned when trying to pop an item from an empty container
    NoItemsError,
}
