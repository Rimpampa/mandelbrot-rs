use super::fragment::FragmentShader as FS;
use super::geometry::GeometryShader as GS;
use super::tess_control::TessControlShader as TCS;
use super::tess_evaluation::TessEvaluationShader as TES;
use super::vertex::VertexShader as VS;

pub struct Program {
    id: u32,
}

impl Program {
    pub fn new(
        vs: &VS,
        tcs: Option<&TCS>,
        tes: Option<&TES>,
        gs: Option<&GS>,
        fs: &FS,
    ) -> Result<Program, String> {
        let id = unsafe { gl::CreateProgram() };
        unsafe {
            gl::AttachShader(id, vs.id());
            gl::AttachShader(id, fs.id());
            if let Some(tcs) = tcs {
                gl::AttachShader(id, tcs.id());
            }
            if let Some(tes) = tes {
                gl::AttachShader(id, tes.id());
            }
            if let Some(gs) = gs {
                gl::AttachShader(id, gs.id());
            }
            gl::LinkProgram(id);
            // Checking program link status
            let mut status = 0;
            gl::GetProgramiv(id, gl::LINK_STATUS, &mut status);

            if status == 0 {
                // Get the legth of the info log
                let mut len = 0;
                gl::GetProgramiv(id, gl::INFO_LOG_LENGTH, &mut len);
                // Allocate the memory to store the log
                let log = super::new_cstring_with_len(len as usize);
                // Retrive the info log
                gl::GetProgramInfoLog(id, len, &mut len, log.as_ptr() as *mut _);
                return if let Ok(string) = log.into_string() {
                    Err(format!("Program linking error:\n{}", string))
                } else {
                    Err("Program linking error:\n<Can't convert the error log to a String>".into())
                };
            }
        }
        Self::unbind();
        Ok(Program { id })
    }

    pub fn uniform_location(&self, name: &str) -> Result<i32, String> {
        let uniform =
            unsafe { gl::GetUniformLocation(self.id, super::string_to_cstring(name).as_ptr()) };
        if uniform < 0 {
            Err(format!("'{}' is not a uniform", name))
        } else {
            Ok(uniform)
        }
    }

    pub fn vertex_attrib_location(&self, name: &str) -> Result<i32, String> {
        let attrib =
            unsafe { gl::GetAttribLocation(self.id, super::string_to_cstring(name).as_ptr()) };
        if attrib < 0 {
            Err(format!("'{}' is not a vertex attribute", name))
        } else {
            Ok(attrib)
        }
    }

    pub fn bind(prg: &Self) {
        unsafe {
            gl::UseProgram(prg.id);
        }
    }

    pub fn unbind() {
        unsafe {
            gl::UseProgram(0);
        }
    }
}
