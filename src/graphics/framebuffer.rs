use super::texture::Texture;

pub struct Framebuffer {
    id: u32,
}

impl Framebuffer {
    pub fn new(tex: &Texture) -> Result<Self, String> {
        let mut id = 0;
        unsafe {
            gl::GenFramebuffers(1, &mut id);
            gl::BindFramebuffer(gl::FRAMEBUFFER, id);
            gl::FramebufferTexture2D(
                gl::FRAMEBUFFER,
                gl::COLOR_ATTACHMENT0,
                gl::TEXTURE_2D,
                tex.id(),
                0,
            );

            let status = gl::CheckFramebufferStatus(gl::FRAMEBUFFER);
            if (status != gl::FRAMEBUFFER_COMPLETE) {
                return Err(format!("Framebuffer incomplete error: {}", status));
            }
        }
        Self::unbind();
        Ok(Framebuffer { id })
    }

    pub fn bind(framebuffer: &Self) {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, framebuffer.id);
        }
    }

    pub fn unbind() {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        }
    }
}

impl Drop for Framebuffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteFramebuffers(1, &mut self.id);
        }
    }
}
