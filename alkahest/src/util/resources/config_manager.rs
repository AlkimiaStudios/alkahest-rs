/// Resource Manager for configurations
pub struct ConfigManager;

/// Contains configuration data
pub struct ConfigResource;

use super::{ResourceHandle, ResourceManager};

impl ResourceManager<ConfigResource> for ConfigManager {
    fn load_resource(_path: String) -> ResourceHandle {
        let h = ResourceHandle{ value: 0 };
        return h;
    }

    fn purge_resource(_handle: ResourceHandle) {

    }

    fn get_resource(_handle: ResourceHandle) -> ConfigResource {
        let r = ConfigResource;
        return r;
    }
}
