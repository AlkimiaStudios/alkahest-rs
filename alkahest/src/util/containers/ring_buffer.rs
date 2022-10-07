use super::ContainerError;

/// A fixed-size ring buffer to be allocated on the stack.
pub struct RingBuffer<T, const N: usize> {
    /// The number of items currently stored in the buffer.
    pub count: usize,
    /// The array containing all of the items currently in the buffer.
    pub items: [T; N],
    /// The front of the buffer, where all pops occur.
    head: usize,
    /// The back of the buffer, where all pushes occur.
    tail: usize,
}

impl<T, const N: usize> RingBuffer<T, N> {
    /// Pushes an item onto the back of the buffer.
    ///
    /// * `item`: T - The item to be added to the buffer.
    ///
    /// -> Result<&T, ContainerError>
    ///
    /// # Examples
    /// ```rust
    /// use alkahest::util::containers::ring_buffer::RingBuffer;
    /// let mut buf: RingBuffer<u32, 10> = RingBuffer::default();
    /// let num: u32 = 1000;
    /// let inserted = buf.push(num).unwrap();
    /// ```
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

    /// Pops an item off the front of the buffer.
    ///
    /// -> Result<&T, ContainerError>
    ///
    /// # Examples
    /// ```rust
    /// use alkahest::util::containers::ring_buffer::RingBuffer;
    /// let mut buf: RingBuffer<u32, 10> = RingBuffer::default();
    /// let num: u32 = 1000;
    /// let inserted = buf.push(num).unwrap();
    /// let popped = buf.pop().unwrap();
    /// ```
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

impl<T, const N: usize> Default for RingBuffer<T, N> where T: Copy + Default {
    fn default() -> RingBuffer<T, N> {
        RingBuffer {
            count: 0usize,
            items: [T::default(); N],
            head: 0usize,
            tail: 0usize
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        let buf: RingBuffer<u32, 10> = RingBuffer::default();
        // No errors means this passes
        assert_eq!(buf.count, 0usize);
    }

    #[test]
    fn test_push() -> Result<(), String> {
        let mut buf: RingBuffer<u32, 10> = RingBuffer::default();
        buf.push(1u32).unwrap();
        assert_eq!(buf.count, 1usize);
        Ok(())
    }

    #[test]
    fn test_pop() -> Result<(), String> {
        let mut buf: RingBuffer<u32, 10> = RingBuffer::default();
        buf.push(1u32).unwrap();
        assert_eq!(buf.pop().unwrap(), &1u32);
        Ok(())
    }

    #[test]
    fn test_push_when_full() -> Result<(), &'static str> {
        let mut buf: RingBuffer<u32, 1> = RingBuffer::default();
        buf.push(1).unwrap();
        match buf.push(1) {
            Ok(_) => Err("RingBuffer.push succeeded when buffer should have been full!"),
            Err(_) => Ok(()),
        }
    }

    #[test]
    fn test_pop_when_empty() -> Result<(), &'static str> {
        let mut buf: RingBuffer<u32, 10> = RingBuffer::default();
        match buf.pop() {
            Ok(_) => Err("RingBuffer.pop succeeded when buffer should have been empty!"),
            Err(_) => Ok(()),
        }
    }
}
