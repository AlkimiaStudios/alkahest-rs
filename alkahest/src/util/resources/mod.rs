mod config_manager;
pub use config_manager::*;

/// Contains the values used for interfacing with resources
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ResourceField {
    /// The index into the resource manager's set of resources
    pub index: u32,
    /// The type of resource the handle represents
    pub resource_type: u32,
}

/// Used to acquire resources from a resource manager.
#[repr(C)]
pub union ResourceHandle {
    /// The underlying resource fields used to access the resource
    pub fields: ResourceField,
    /// The overall value of the handle, used for comparison operations
    pub value: u64,
}

trait ResourceManager<T> {
    fn load_resource(path: String) -> ResourceHandle;
    fn purge_resource(handle: ResourceHandle);
    fn get_resource(handle: ResourceHandle) -> T;
}
