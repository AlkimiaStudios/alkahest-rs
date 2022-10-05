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
    /// let buf: RingBuffer<u32, 10> = RingBuffer;
    /// let num: u32 = 1000;
    /// let inserted = buf.push(num)?;
    /// Ok()
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
    /// let buf: RingBuffer<u32, 10> = RingBuffer;
    /// let num: u32 = 1000;
    /// let inserted = buf.push(num)?;
    /// let popped = buf.pop()?;
    /// Ok()
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
