/// Contains the RingBuffer<T, const N: usize> struct
pub mod ring_buffer;

/// Errors used by container structs
#[derive(Debug)]
#[allow(dead_code)]
pub enum ContainerError {
    /// Error returned when trying to allocate space in a full container
    NoSpaceError,
    /// Error returned when trying to pop an item from an empty container
    NoItemsError,
}
