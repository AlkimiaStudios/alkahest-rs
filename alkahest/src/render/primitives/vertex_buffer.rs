extern crate gl;
use std::marker::PhantomData;

#[derive(Debug)]
pub(crate) struct VertexBuffer<T> {
    pub id: u32,
    data: PhantomData<T>,
}

impl<T> VertexBuffer<T> {
    pub unsafe fn new(vertices: Vec<T>) -> Self {
        let mut id: u32 = 0;
        gl::GenBuffers(1, &mut id);

        gl::BindBuffer(gl::ARRAY_BUFFER, id);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<T>()) as isize,
            vertices.as_ptr().cast(),
            gl::STATIC_DRAW);
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);

        VertexBuffer { id, data: PhantomData }
    }

    pub unsafe fn new_from_arr(vertices: &[T]) -> Self {
        let mut id: u32 = 0;
        gl::GenBuffers(1, &mut id);

        gl::BindBuffer(gl::ARRAY_BUFFER, id);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<T>()) as isize,
            vertices.as_ptr().cast(),
            gl::STATIC_DRAW);
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);

        VertexBuffer { id, data: PhantomData }
    }

    pub unsafe fn dynamic_new(count: usize) -> Self {
        let mut id: u32 = 0;
        gl::GenBuffers(1, &mut id);

        gl::BindBuffer(gl::ARRAY_BUFFER, id);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (count * std::mem::size_of::<T>()) as isize,
            0 as *const _,
            gl::DYNAMIC_DRAW);
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);

        VertexBuffer { id, data: PhantomData }
    }

    pub unsafe fn bind(&self) {
        gl::BindBuffer(gl::ARRAY_BUFFER, self.id);
    }

    pub unsafe fn set_data(&self, data: &Vec<T>, count: usize, offset: usize) {
        self.bind();
        gl::BufferSubData(gl::ARRAY_BUFFER, offset as isize, (count * std::mem::size_of::<T>()) as isize, data.as_ptr().cast());
        self.unbind();
    }

    pub unsafe fn unbind(&self) {
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    }

    pub unsafe fn destroy(&self) {
        gl::DeleteBuffers(1, self.id as *const u32);
    }
}
