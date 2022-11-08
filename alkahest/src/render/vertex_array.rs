extern crate gl;
use std::ffi::c_void;
use super::VertexBuffer;

pub(crate) struct VertexArray {
    pub id: u32,
}

impl VertexArray {
    pub unsafe fn new() -> Self {
        let mut id: u32 = 0;
        gl::CreateVertexArrays(1, &mut id);
        VertexArray { id }
    }

    pub unsafe fn link_attributes<T>(&self,
        vbo: &VertexBuffer<T>,
        layout: u32,
        num_components: i32,
        attrib_type: u32,
        stride: i32,
        offset: *const c_void
    ) {
        self.bind();
        vbo.bind();
        gl::VertexAttribPointer(layout, num_components, attrib_type, gl::FALSE, stride, offset);
        gl::EnableVertexAttribArray(layout);

        vbo.unbind();
        self.unbind();
    }

    pub unsafe fn bind(&self) {        
        gl::BindVertexArray(self.id);   
    }

    pub unsafe fn unbind(&self) {
        gl::BindVertexArray(0);
    }

    pub unsafe fn destroy(&self) {
        gl::DeleteVertexArrays(1, self.id as *const u32);
    }
}
