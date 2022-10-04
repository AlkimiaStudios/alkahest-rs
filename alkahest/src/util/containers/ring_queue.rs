use super::ContainerError;

pub struct RingQueue<T, const N: usize> {
    pub count: usize,
    pub items: [T; N],
    head: usize,
    tail: usize,
}

impl<T, const N: usize> RingQueue<T, N> {
    pub fn push(&mut self, item: T) -> Result<&T, ContainerError> {
        if self.count == N {
            return Err(ContainerError::NoSpaceError);
        }
        self.items[self.tail] = item;
        self.count = self.count + 1;
        self.tail = self.tail + 1;
        if self.tail == N {
            self.tail = 0;
        }

        Ok(&self.items[self.head])
    }

    pub fn pop(&mut self) -> Result<&T, ContainerError> {
        if self.count == 0 {
            Err(ContainerError::NoItemsError)
        } else {
            let item = &self.items[self.head];
            self.count = self.count - 1;
            self.head = self.head + 1;
            if self.head == N {
                self.head = 0;
            }
            Ok(item)
        }
    }
}
