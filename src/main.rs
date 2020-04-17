#[allow(unused)]
mod graphics;

use graphics::{shader::*, vao::VertexArrayObject};

use sdl2::{
    event::{Event, WindowEvent},
    keyboard::{Mod, Scancode},
    mouse::*,
};

use std::path::Path;

fn aspect_ratio(width: i32, height: i32) -> (f32, f32) {
    if width > height {
        (width as f32 / height as f32, 1.0)
    } else {
        (1.0, height as f32 / width as f32)
    }
}

fn main() -> Result<(), String> {
    let sdl = sdl2::init()?; // Initialize sdl2 crate
    let video_subsystem = sdl.video()?; // Get the video subsystem

    // Setup some opengl attributes
    let gl_attr = video_subsystem.gl_attr(); // Get the attributes
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core); // Profile
    gl_attr.set_context_version(4, 0); // Version

    // Create a new window
    let window = video_subsystem
        .window("Mandelbrot", 500, 500)
        .resizable()
        .maximized()
        .opengl()
        .build()
        .unwrap();

    // Make it the current opengl context
    let _glcontext = window.gl_create_context()?;

    // Load OpenGL functions
    let _gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const _);

    // Load the vertex shader form the file and compile it
    let vs_fractal = VertexShader::new(Path::new("shaders/fractal.vert"))?;

    // Load the fragment shader form the file and compile it
    let fs_fractal = FragmentShader::new(Path::new("shaders/dfractal.frag"))?;

    // Link the shaders to the program and compile it
    let fractal_program = Program::new(&vs_fractal, None, None, None, &fs_fractal)?;
    Program::bind(&fractal_program);

    // Retrive the location of the uniforms
    let aspect_loc = fractal_program.uniform_location("Aspect")?;
    let offset_loc = fractal_program.uniform_location("Offset")?;
    let iter_loc = fractal_program.uniform_location("iIterations")?;
    let zoom_loc = fractal_program.uniform_location("Zoom")?;
    let size_loc = fractal_program.uniform_location("iSize").unwrap_or(-1);

    // Initial window size
    let window_size = window.size();
    unsafe {
        // Send the initial size to OpenGL
        gl::Uniform2i(size_loc, window_size.0 as i32, window_size.1 as i32);
    }

    // Tuple holding the normalized width and height of the window
    let mut aspect = aspect_ratio(window_size.0 as i32, window_size.1 as i32);
    unsafe {
        // Send the initial size to OpenGL
        gl::Uniform2f(aspect_loc, aspect.0, aspect.1);
    }
    // Tuple holding the offset from the center
    let mut offset = (-0.75, 0.0);
    unsafe {
        // Send the initial offset to OpenGL
        gl::Uniform2d(offset_loc, offset.0, offset.1);
    }
    let mut px_size = (
        aspect.0 / window_size.0 as f32 * 2.0,
        aspect.1 / window_size.1 as f32 * 2.0,
    );
    // Number of iterations to compute
    let mut iterations = 1;
    unsafe {
        // Send the initial number of maximum iterations to OpenGL
        gl::Uniform1i(iter_loc, iterations);
    }
    // Amount of zoom
    let mut zoom = 1.0;
    unsafe {
        // Send the initial zoom to OpenGL
        gl::Uniform1f(zoom_loc, zoom);
    }

    // Create and bind a vertex array object
    let vao = VertexArrayObject::new();
    VertexArrayObject::bind(&vao);

    // Generate an event pump
    let mut event_pump = sdl.event_pump()?;

    // Flag that indicates whether or not the left mouse button is being pressed
    let mut pressing = false;
    // Flag that indicates if the screen needs to be updated
    let mut update = true;

    // Start the main loop
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                // Window is closing
                Event::Quit { .. } => return Ok(()),

                // ----- WINDOW EVENTS -----
                Event::Window { win_event, .. } => match win_event {
                    WindowEvent::Resized(width, height) => {
                        // Normalize the size of the window
                        aspect = aspect_ratio(width, height);
                        // Calculate the new pixel size
                        px_size.0 = aspect.0 / width as f32 * 2.0;
                        px_size.1 = aspect.1 / height as f32 * 2.0;
                        unsafe {
                            // Update the OpenGL viewport
                            gl::Viewport(0, 0, width, height);
                            // Send the normalized size to OpenGL
                            gl::Uniform2f(aspect_loc, aspect.0, aspect.1);
                            gl::Uniform2i(size_loc, width, height);
                        }
                        update = true;
                    }
                    _ => {}
                },
                // ----- KEYBOARD EVENTS ------

                // A key was pressed
                Event::KeyDown {
                    scancode: Some(key),
                    keymod: k,
                    ..
                } => match key {
                    // The plus(+) was pressed
                    Scancode::KpPlus => {
                        if k.intersects(Mod::LCTRLMOD | Mod::RCTRLMOD) {
                            iterations += 100;
                        } else {
                            iterations += 1;
                        }
                        unsafe {
                            gl::Uniform1i(iter_loc, iterations);
                        }
                        update = true;
                    }
                    // The minus(-) was pressed
                    Scancode::KpMinus if iterations > 1 => {
                        if k.intersects(Mod::LCTRLMOD | Mod::RCTRLMOD) {
                            if iterations > 100 {
                                iterations -= 100;
                            } else {
                                iterations = 1;
                            }
                        } else {
                            iterations -= 1;
                        }
                        unsafe {
                            gl::Uniform1i(iter_loc, iterations);
                        }
                        update = true;
                    }
                    _ => {}
                },
                // ----- MOUSE EVENTS -----

                // A mouse button is being pressed
                Event::MouseButtonDown { mouse_btn, .. } => match mouse_btn {
                    // If it's the left button
                    MouseButton::Left => pressing = true,
                    _ => {}
                },
                // A mouse button is being released
                Event::MouseButtonUp { mouse_btn, .. } => match mouse_btn {
                    // If it's the left button
                    MouseButton::Left => pressing = false,
                    _ => {}
                },
                // The user is dragging
                Event::MouseMotion { xrel, yrel, .. } if pressing => {
                    // Calculate the new offset
                    offset.0 -= xrel as f64 * px_size.0 as f64 / zoom as f64;
                    offset.1 += yrel as f64 * px_size.1 as f64 / zoom as f64;
                    unsafe {
                        // Update the offset uniform
                        gl::Uniform2d(offset_loc, offset.0, offset.1);
                    }
                    update = true;
                }
                // The mouse wheel is being scrolled
                Event::MouseWheel { y, .. } => {
                    // Increase/decrase the zoom
                    zoom += y as f32 * zoom * 0.1;
                    unsafe {
                        // Update the zoom uniform
                        gl::Uniform1f(zoom_loc, zoom);
                    }
                    update = true;
                }
                _ => {}
            }
        }
        if update {
            unsafe {
                gl::Clear(gl::COLOR_BUFFER_BIT);
                gl::DrawArrays(gl::TRIANGLES, 0, 6);
            }
            window.gl_swap_window();
            update = false;
            // DEBUG MESSAGES
            // println!("Iterations: {}", iterations);
            // println!("Offset: {:4}, {:4}", offset.0, offset.1);
            // println!("Zoom: {}", zoom);
            // println!("Window: {:4}, {:4}", window.size().0, window.size().1);
            // println!("Aspect: {:4}, {:4}", aspect.0, aspect.1);
        }
    }
}
