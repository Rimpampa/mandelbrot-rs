use std::ffi::c_void;
use std::path::Path;
use std::ptr;

use gl;
use gl::types as gl_t;

pub struct Texture {
    id: u32,
    width: u32,
    height: u32,
    px_size: (f32, f32),
}

impl Texture {
    pub fn new(width: u32, height: u32) -> Self {
        let mut id: gl_t::GLuint = 0;
        unsafe {
            // Genereate a new texture
            gl::GenTextures(1, &mut id);
            gl::BindTexture(gl::TEXTURE_2D, id);

            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGB as i32,
                width as i32,
                height as i32,
                0,
                gl::RGB,
                gl::UNSIGNED_BYTE,
                ptr::null(),
            );
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
        }
        Self::unbind();
        Texture {
            id,
            width,
            height,
            px_size: (1.0 / width as f32, 1.0 / height as f32),
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if self.width != width || self.height != height {
            Self::bind(self);

            self.width = width;
            self.height = height;
            self.px_size = (1.0 / width as f32, 1.0 / height as f32);
            unsafe {
                gl::TexImage2D(
                    gl::TEXTURE_2D,
                    0,
                    gl::RGB as i32,
                    width as i32,
                    height as i32,
                    0,
                    gl::RGB,
                    gl::UNSIGNED_BYTE,
                    ptr::null(),
                );
            }
            Self::unbind();
        }
    }

    pub fn get_image(&self) -> Vec<u8> {
        Self::bind(self);

        let size = self.width as usize * self.height as usize * 3;
        let mut vec = Vec::with_capacity(size);
        unsafe {
            gl::GetTexImage(
                gl::TEXTURE_2D,
                0,
                gl::RGB,
                gl::UNSIGNED_BYTE,
                vec.as_mut_ptr() as _,
            );
        }
        Self::unbind();
        vec
    }

    pub fn bind(tex: &Self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, tex.id);
        }
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn pixel_size(&self) -> (f32, f32) {
        self.px_size
    }

    pub fn set_active_unit(unit: u32) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + unit);
        }
    }

    pub fn unbind() {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }
}
