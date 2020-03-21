use wasm_bindgen::JsCast;
use web_sys::WebGlRenderingContext as GL;
use web_sys::*;
use js_sys::WebAssembly;
use super::super::common_funcs as cf;

pub struct Color2D {
    program: WebGlProgram,
    rect_vertice_buffer: WebGlBuffer,
}

impl Color2D {
    pub fn new(gl: &WebGlRenderingContext) -> Self {
        let program = cf::link_program(
            &gl,
            super::super::shaders::vertex::color_2d::SHADER,
            super::super::shaders::fragment::color_2d::SHADER,
        ).unwrap();

        let vertices_rect: [f32; 12] = [
            0., 1., // x, y
            0., 0., // x, y
            1., 1., // x, y
            1., 1., // x, y
            0., 0., // x, y
            1., 0., // x, y
        ];

        let memory_buffer = wasm_bindgen::memory()
            .dyn_into::<WebAssembly::Memory>()
            .unwrap()
            .buffer();
        let vertices_location = vertices_rect.as_ptr() as u32 / 4;
        let vert_array = js_sys::Float32Array::new(&memory_buffer).subarray(
            vertices_location,
            vertices_location + vertices_rect.len() as u32,
        );
        let buffer_rect = gl.create_buffer().ok_or("failed to create buffer").unwrap();
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&buffer_rect));
        gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &vert_array, GL::STATIC_DRAW);

        Self {
            program: program,
            rect_vertice_buffer: buffer_rect,
        }
    }
}

