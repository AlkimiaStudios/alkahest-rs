mod ring_buffer;
mod handle_map;

pub use ring_buffer::RingBuffer;
pub use handle_map::*;

/// Errors used by container structs
#[derive(Debug)]
pub enum ContainerError {
    /// Error returned when trying to allocate space in a full container
    NoSpaceError(),
    /// Error returned when trying to pop an item from an empty container
    NoItemsError(),
    /// Error returned when the value is actually None
    MissingValueError(),
}

impl std::fmt::Display for ContainerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContainerError::NoSpaceError() =>
                write!(f, "No space remaining in container!"),
            ContainerError::NoItemsError() =>
                write!(f, "No items present in container!"),
            ContainerError::MissingValueError() =>
                write!(f, "Requested index was None!"),
        }
    }
}

impl std::error::Error for ContainerError {}
