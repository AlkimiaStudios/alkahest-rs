extern crate gl;

pub(crate) struct IndexBuffer {
    pub id: u32,
}

impl IndexBuffer {
    pub unsafe fn new(indices: Vec<u32>) -> Self {
        let mut id: u32 = 0;
        gl::GenBuffers(1, &mut id);

        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, id);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            (indices.len() * 4) as isize,
            indices.as_ptr().cast(),
            gl::STATIC_DRAW);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);

        IndexBuffer { id }
    }

    pub unsafe fn bind(&self) {
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.id);
    }

    pub unsafe fn unbind(&self) {
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
    }

    pub unsafe fn destroy(&self) {
        gl::DeleteBuffers(1, self.id as *const u32);
    }
}
