use super::ContainerError;
use modular_bitfield::prelude::*;

/// Handle used to access items in a HandleMap
#[bitfield]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Handle {
    index: B32,
    generation: B16,
    item_type: B15,
    free: B1,
}

#[derive(Debug)]
struct Meta {
    dense_to_sparse: u32,
}

/// A map that uses a dense set to maintain cache locality
#[derive(Debug)]
pub struct HandleMap<T> {
    items: Vec<T>,
    handles: Vec<Handle>,
    meta: Vec<Meta>,
    item_type: u16,
    fragmented: bool,
    freelist_front: u32,
    freelist_back: u32,
}

impl<T> HandleMap<T> where T: Clone {
    /// Creates a new HandleMap with space for `reserve_count` items
    pub fn new(item_type: u16, reserve_count: usize) -> HandleMap<T> {
        let mut h = HandleMap {
            items: vec![],
            handles: vec![],
            meta: vec![],
            item_type,
            fragmented: false,
            freelist_front: 0xFFFFFFFF,
            freelist_back: 0xFFFFFFFF
        };

        h.items.reserve(reserve_count);
        h.handles.reserve(reserve_count);
        h.meta.reserve(reserve_count);

        return h;
    }

    /// Returns the number of items stored in the dense set
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Inserts an item into the map, returning a Handle for later access
    pub fn insert(&mut self, item: T) -> Handle {
        let mut h: Handle;
        self.fragmented = true;

        if self.freelist_empty() {
            let mut inner_id = Handle::new();
            inner_id.set_index(self.items.len() as u32);
            inner_id.set_generation(1);
            inner_id.set_item_type(self.item_type);

            h = inner_id;
            h.set_index(self.handles.len() as u32);

            self.handles.push(inner_id);
        } else {
            let outer_index = self.freelist_front;
            let mut inner_id = self.handles[outer_index as usize];

            self.freelist_front = inner_id.index();
            if self.freelist_empty() {
                self.freelist_back = self.freelist_front;
            }

            inner_id.set_index(self.items.len() as u32);
            h = inner_id;
            h.set_index(outer_index);
        }

        self.items.push(item);
        self.meta.push(Meta { dense_to_sparse: h.index() });

        return h;
    }

    /// Returns an item from the map, given a Handle
    pub fn get(&self, handle: Handle) -> Result<T, ContainerError> {
        self.is_valid(handle)?;

        let inner_id = self.handles[handle.index() as usize];
        let item = self.items.get(inner_id.index() as usize).ok_or(ContainerError::MissingValueError())?;
        Ok(item.clone())
    }

    /// Removes an item from the map
    pub fn erase(&mut self, handle: Handle) -> Result<(), ContainerError> {
        self.is_valid(handle)?;

        self.fragmented = true;

        let mut inner_id = self.handles[handle.index() as usize];
        let inner_index = inner_id.index();

        inner_id.set_free(1);
        inner_id.set_generation(inner_id.generation() + 1);
        inner_id.set_index(0xFFFFFFFF);  // Max value here == end of freelist
        self.handles[handle.index() as usize] = inner_id;

        if self.freelist_empty() {
            self.freelist_front = handle.index();
            self.freelist_back = self.freelist_front;
        } else {
            // Set previous back of freelist to new back
            self.handles[self.freelist_back as usize].set_index(handle.index());
            self.freelist_back = handle.index();
        }

        // Remove the item by swapping with the last element, then pop back
        if inner_index != (self.items.len() as u32) - 1 {
            self.items[inner_index as usize] = self.items.pop().unwrap();
            self.meta[inner_index as usize] = self.meta.pop().unwrap();

            // Fix the inner index of the swapped item
            self.handles[self.meta[inner_index as usize].dense_to_sparse as usize].set_index(inner_index);
        }

        Ok(())
    }

    //TODO: Implement clear and defragment functions

    fn freelist_empty(&self) -> bool {
        self.freelist_front == 0xFFFFFFFF
    }

    fn is_valid(&self, handle: Handle) -> Result<(), ContainerError> {
        if handle.index() >= self.handles.len() as u32 {
            return Err(ContainerError::InvalidHandleError());
        }

        let inner_id = self.handles.get(handle.index() as usize).ok_or(ContainerError::InvalidHandleError())?;
        let valid = {
            inner_id.index() < self.items.len() as u32
            && handle.item_type() == self.item_type
            && handle.generation() == inner_id.generation()
        };

        if valid {
            Ok(())
        } else {
            Err(ContainerError::InvalidHandleError())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::trace;

    #[test]
    fn test_init() {
        let map: HandleMap<u32> = HandleMap::new(0, 10);
        assert_eq!(map.len(), 0usize);
    }

    #[test]
    fn test_insert() {
        let mut map: HandleMap<u32> = HandleMap::new(0, 10);
        let h = map.insert(1u32);
        assert_eq!(h, Handle::new().with_generation(1));
        assert_eq!(map.len(), 1usize);
    }

    #[test]
    fn test_get() {
        let mut map: HandleMap<u32> = HandleMap::new(0, 10);
        let h = map.insert(9999u32);
        let value = map.get(h).unwrap();
        assert_eq!(value, 9999u32)
    }

    #[test]
    fn test_erase() -> Result<(), &'static str> {
        let mut map: HandleMap<u32> = HandleMap::new(0, 10);
        let h = map.insert(9999u32);
        map.erase(h).unwrap();
        match map.get(h) {
            Ok(_) => Err("HandleMap.get succeeded when Handle should have been invalidated!"),
            Err(e) => Ok(()),
        }
    }
}
