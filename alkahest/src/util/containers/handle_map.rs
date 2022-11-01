use bitfield::bitfield;

bitfield! {
    /// Used to access items in a HandleMap
    #[derive(Copy, Clone)]
    pub struct Handle(u64);
    impl Debug;
    u32, index, set_index: 32, 0; 
    u16, generation, set_generation: 16, 0;
    u16, item_type, set_item_type: 16, 1;
    u16, free, set_free: 0;
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

    /// Inserts an item into the map, returning a Handle for later access
    pub fn insert(&mut self, item: T) -> Handle {
        let mut h: Handle;
        self.fragmented = true;

        if self.freelist_empty() {
            let mut inner_id = Handle(0);
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
    pub fn get(&self, handle: Handle) -> T {
        //TODO: Add range validation
        let inner_id = self.handles[handle.index() as usize];
        return self.items[inner_id.index() as usize].clone();
    }

    /// Removes an item from the map
    pub fn erase(&mut self, handle: Handle) {
        if !self.is_valid(handle) {
            return;
        }

        self.fragmented = true;

        let mut inner_id = self.handles[handle.index() as usize];
        let inner_index = inner_id.index();

        inner_id.set_free(true);
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
    }

    //TODO: Implement clear and defragment functions

    fn freelist_empty(&self) -> bool {
        self.freelist_front == 0xFFFFFFFF
    }

    fn is_valid(&self, handle: Handle) -> bool {
        if handle.index() >= self.handles.len() as u32 {
            return false;
        }

        let inner_id = self.handles[handle.index() as usize];
        return {
            inner_id.index() < self.items.len() as u32
            && handle.item_type() == self.item_type
            && handle.generation() == inner_id.generation()
        }
    }
}
