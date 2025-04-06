#![no_std]
#![no_main]

use wut::prelude::*;
use wut::*;

use wut::gx2::shader::{self, attribute::Float4};

#[derive(ShaderAttributes)]
struct MyShader {
    #[name = "aPosition"]
    // #[index = 0]
    // #[offset = 3]
    a_position: shader::Attribute<Float4>,
    #[name = "aColour"]
    // #[index = 1]
    // #[offset = 0]
    a_color: shader::Attribute<Float4>,
}

static PROGRAM: shader::Program = shader::Program::from(include_bytes!("shader.gsh"));

/*
&[
    0x47, 0x66, 0x78, 0x32, 0x00, 0x00, 0x00, 0x20, 0x00, 0x00, 0x00, 0x07, 0x00, 0x00, 0x00, 0x01,
    0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x42, 0x4c, 0x4b, 0x7b, 0x00, 0x00, 0x00, 0x20, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x01, 0x9c, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x01, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01,
    0xff, 0xff, 0x01, 0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00, 0xff, 0xff, 0xff, 0xfc,
    0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0xff,
    0x00, 0x00, 0x00, 0xff, 0x00, 0x00, 0x00, 0xff, 0x00, 0x00, 0x00, 0xff, 0x00, 0x00, 0x00, 0xff,
    0x00, 0x00, 0x00, 0xff, 0x00, 0x00, 0x00, 0xff, 0x00, 0x00, 0x00, 0xff, 0x00, 0x00, 0x00, 0xff,
    0x00, 0x00, 0x00, 0xff, 0x00, 0x00, 0x00, 0xff, 0x00, 0x00, 0x00, 0xff, 0x00, 0x00, 0x00, 0xff,
    0x00, 0x00, 0x00, 0xff, 0x00, 0x00, 0x00, 0xff, 0x00, 0x00, 0x00, 0xff, 0x00, 0x00, 0x00, 0xff,
    0x00, 0x00, 0x00, 0xff, 0x00, 0x00, 0x00, 0xff, 0x00, 0x00, 0x00, 0xff, 0x00, 0x00, 0x00, 0xff,
    0x00, 0x00, 0x00, 0xff, 0x00, 0x00, 0x00, 0xff, 0x00, 0x00, 0x00, 0xff, 0x00, 0x00, 0x00, 0xff,
    0x00, 0x00, 0x00, 0xff, 0x00, 0x00, 0x00, 0xff, 0x00, 0x00, 0x00, 0xff, 0x00, 0x00, 0x00, 0xff,
    0x00, 0x00, 0x00, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0e, 0x00, 0x00, 0x00, 0x10,
    0x00, 0x00, 0x01, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0xd0, 0x60, 0x01, 0x34, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0xca, 0x70, 0x01, 0x54, 0x00, 0x00, 0x00, 0x0b, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0xca, 0x70, 0x01, 0x5c, 0x00, 0x00, 0x00, 0x0b, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x01, 0x61, 0x43, 0x6f, 0x6c, 0x6f, 0x75, 0x72, 0x00, 0x61, 0x50, 0x6f, 0x73,
    0x69, 0x74, 0x69, 0x6f, 0x6e, 0x00, 0x00, 0x00, 0xd0, 0x60, 0x01, 0x08, 0xca, 0x70, 0x01, 0x34,
    0xca, 0x70, 0x01, 0x44, 0x7d, 0x42, 0x4c, 0x4b, 0x00, 0x00, 0x00, 0x28, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x01, 0x68, 0xd0, 0x60, 0x00, 0x00, 0x00, 0x00, 0x00, 0x14, 0xd0, 0x60, 0x01, 0x54,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 0xd0, 0x60, 0x01, 0x68, 0x42, 0x4c, 0x4b, 0x7b,
    0x00, 0x00, 0x00, 0x20, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x05,
    0x00, 0x00, 0x01, 0x08, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x80, 0x09, 0x3c, 0x20, 0x01, 0x00, 0x88, 0x06, 0x00, 0x94, 0x00, 0xc0, 0x00, 0x00,
    0x88, 0x06, 0x00, 0x14, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xa0, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80,
    0x00, 0x0d, 0x00, 0x00, 0x42, 0x4c, 0x4b, 0x7b, 0x00, 0x00, 0x00, 0x20, 0x00, 0x00, 0x00, 0x01,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x06, 0x00, 0x00, 0x01, 0x10, 0x00, 0x00, 0x00, 0x02,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x02, 0x14, 0x00, 0x00, 0x01,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0f, 0x00, 0x00, 0x00, 0x01,
    0x00, 0x00, 0x00, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x58, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x7d, 0x42, 0x4c, 0x4b,
    0x00, 0x00, 0x00, 0x28, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xe8, 0xd0, 0x60, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0xd0, 0x60, 0x00, 0xe8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0xd0, 0x60, 0x00, 0xe8, 0x42, 0x4c, 0x4b, 0x7b, 0x00, 0x00, 0x00, 0x20, 0x00, 0x00, 0x00, 0x01,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x07, 0x00, 0x00, 0x01, 0x58, 0x00, 0x00, 0x00, 0x03,
    0x00, 0x00, 0x00, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x28, 0xa0, 0x00, 0x00, 0x00, 0x00,
    0x88, 0x06, 0x20, 0x94, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x90, 0x0c, 0x00, 0x20, 0x00, 0x08, 0x00, 0x00,
    0x90, 0x0c, 0x00, 0x40, 0x00, 0xa0, 0x1f, 0x00, 0xfd, 0x04, 0x62, 0x6f, 0x00, 0x0c, 0x00, 0x80,
    0x90, 0x0c, 0x00, 0x60, 0x83, 0xf9, 0x22, 0x3e, 0x00, 0x00, 0x00, 0x3f, 0xfe, 0x0c, 0x00, 0x80,
    0x00, 0x08, 0x00, 0x40, 0xfe, 0xa8, 0x9f, 0x80, 0xfd, 0x00, 0x62, 0x2f, 0xdb, 0x0f, 0xc9, 0x40,
    0xdb, 0x0f, 0x49, 0xc0, 0xfe, 0xa4, 0x1f, 0x80, 0x80, 0x00, 0x00, 0x00, 0x83, 0xf9, 0x22, 0x3e,
    0x00, 0x00, 0x00, 0x00, 0xfe, 0x00, 0x00, 0x80, 0x10, 0x37, 0x00, 0x00, 0x42, 0x4c, 0x4b, 0x7b,
    0x00, 0x00, 0x00, 0x20, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x00,
]
*/

#[wut_main(Cafe)]
fn main() {
    use wut::bindings::*;

    let mut shader: shader::Shader<MyShader> = shader::Shader::new(0, &PROGRAM).unwrap();

    let mut position_buffer = shader::Buffer::from(
        &[
            Float4::from((1.0, -1.0, 0.0, 1.0)),
            Float4::from((0.0, 1.0, 0.0, 1.0)),
            Float4::from((-1.0, -1.0, 1.0, 1.0)),
        ],
        shader::buffer::ResourceFlags::BindVertexBuffer
            | shader::buffer::ResourceFlags::UsageCPU
            | shader::buffer::ResourceFlags::UsageGPURead,
    )
    .unwrap();

    let mut color_buffer = shader::Buffer::from(
        &[
            Float4::from((1.0, 0.0, 0.0, 1.0)),
            Float4::from((0.0, 1.0, 0.0, 1.0)),
            Float4::from((0.0, 0.0, 1.0, 1.0)),
        ],
        shader::buffer::ResourceFlags::BindVertexBuffer
            | shader::buffer::ResourceFlags::UsageCPU
            | shader::buffer::ResourceFlags::UsageGPURead,
    )
    .unwrap();

    let context = gx2::context::RenderContext::new();

    let mut frame = 0.0;

    while process::running() {
        frame += 0.05;

        {
            let mut b = color_buffer.lock().unwrap();
            let i = 0;
            b[i].x = 1.0;
            // b[i].y = if b[i].y >= 1.0 { 0.0 } else { b[i].y + 0.01 };
            // b[i].z = if b[i].z >= 1.0 { 0.0 } else { b[i].z + 0.01 };
            b[i].y = 0.5 * (1.0 + frame.sin());
            b[i].z = 0.5 * (1.0 + frame.cos());
            b[i].w = 1.0;

            // let i = 1;
            // b[i].x = if b[i].x >= 1.0 { 0.0 } else { b[i].x + 0.01 };
            // b[i].y = 1.0;
            // b[i].z = if b[i].z >= 1.0 { 0.0 } else { b[i].z + 0.01 };
            // b[i].w = 1.0;

            // let i = 2;
            // b[i].x = if b[i].x >= 1.0 { 0.0 } else { b[i].x + 0.01 };
            // b[i].y = if b[i].y >= 1.0 { 0.0 } else { b[i].y + 0.01 };
            // b[i].z = 1.0;
            // b[i].w = 1.0;
        }

        let context = context.ready().tv();

        gx2::screen::fill(&context, gx2::color::Color::blue());

        shader
            .attributes
            .a_position
            .set_buffer(&mut position_buffer);
        shader.attributes.a_color.set_buffer(&mut color_buffer);

        shader.render();

        let context = context.drc();

        gx2::screen::fill(&context, gx2::color::Color::magenta());

        context.finish();
    }

    // unsafe {
    //     /*
    //     let mut group = WHBGfxShaderGroup::default();

    //     if WHBGfxLoadGFDShaderGroup(&mut group, 0, SHADER.as_ptr() as *const _) == 0 {
    //         return;
    //     }
    //     */

    //     // let group = ShaderGroup::new(0, &SHADER).unwrap();

    //     // WHBGfxInitShaderAttribute(
    //     //     &mut group,
    //     //     c"aPosition".as_ptr(),
    //     //     0,
    //     //     0,
    //     //     GX2AttribFormat::GX2_ATTRIB_FORMAT_FLOAT_32_32_32_32,
    //     // );
    //     // WHBGfxInitShaderAttribute(
    //     //     &mut group,
    //     //     c"aColour".as_ptr(),
    //     //     1,
    //     //     0,
    //     //     GX2AttribFormat::GX2_ATTRIB_FORMAT_FLOAT_32_32_32_32,
    //     // );
    //     // WHBGfxInitFetchShader(&mut group);

    //     /*
    //     {
    //         use GX2RResourceFlags as F;

    //         position_buffer.flags = F::GX2R_RESOURCE_BIND_VERTEX_BUFFER
    //             | F::GX2R_RESOURCE_USAGE_CPU_READ
    //             | F::GX2R_RESOURCE_USAGE_CPU_WRITE
    //             | F::GX2R_RESOURCE_USAGE_GPU_READ;
    //         position_buffer.elemSize = 4 * 4;
    //         position_buffer.elemCount = 3;
    //         GX2RCreateBuffer(&mut position_buffer);
    //         let buffer = GX2RLockBufferEx(&mut position_buffer, 0) as *mut f32;
    //         buffer.copy_from(
    //             [
    //                 1.0, -1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, -1.0, -1.0, 1.0, 1.0,
    //             ]
    //             .as_ptr(),
    //             4 * 3,
    //         );
    //         GX2RUnlockBufferEx(&mut position_buffer, 0);

    //         color_buffer.flags = F::GX2R_RESOURCE_BIND_VERTEX_BUFFER
    //             | F::GX2R_RESOURCE_USAGE_CPU_READ
    //             | F::GX2R_RESOURCE_USAGE_CPU_WRITE
    //             | F::GX2R_RESOURCE_USAGE_GPU_READ;
    //         color_buffer.elemSize = 4 * 4;
    //         color_buffer.elemCount = 3;
    //         GX2RCreateBuffer(&mut color_buffer);
    //         let buffer = GX2RLockBufferEx(&mut color_buffer, 0) as *mut f32;
    //         buffer.copy_from(
    //             [1.0, 0.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0].as_ptr(),
    //             4 * 3,
    //         );
    //         GX2RUnlockBufferEx(&mut color_buffer, 0);
    //     }
    //     */
    //     while process::running() {
    //         /*
    //         {
    //             let colors = core::slice::from_raw_parts_mut(
    //                 GX2RLockBufferEx(&mut color_buffer, 0) as *mut f32,
    //                 12,
    //             );
    //             colors[0] = 1.0;
    //             colors[1] = if colors[1] >= 1.0 {
    //                 0.0
    //             } else {
    //                 colors[1] + 0.01
    //             };
    //             colors[2] = if colors[2] >= 1.0 {
    //                 0.0
    //             } else {
    //                 colors[2] + 0.01
    //             };
    //             colors[3] = 1.0;
    //             // ------------------------------
    //             colors[4] = if colors[4] >= 1.0 {
    //                 0.0
    //             } else {
    //                 colors[4] + 0.01
    //             };
    //             colors[5] = 1.0;
    //             colors[6] = if colors[6] >= 1.0 {
    //                 0.0
    //             } else {
    //                 colors[6] + 0.01
    //             };
    //             colors[7] = 1.0;
    //             // ------------------------------
    //             colors[8] = if colors[8] >= 1.0 {
    //                 0.0
    //             } else {
    //                 colors[8] + 0.01
    //             };
    //             colors[9] = if colors[9] >= 1.0 {
    //                 0.0
    //             } else {
    //                 colors[9] + 0.01
    //             };
    //             colors[10] = 1.0;
    //             colors[11] = 1.0;

    //             GX2RUnlockBufferEx(&mut color_buffer, 0);
    //         }
    //          */

    //         /*
    //         {
    //             let mut b = color_buffer.lock().unwrap();
    //             let i = 0;
    //             b[i].x = 1.0;
    //             b[i].y = if b[i].y >= 1.0 { 0.0 } else { b[i].y + 0.01 };
    //             b[i].z = if b[i].z >= 1.0 { 0.0 } else { b[i].z + 0.01 };
    //             b[i].w = 1.0;

    //             let i = 1;
    //             b[i].x = if b[i].x >= 1.0 { 0.0 } else { b[i].x + 0.01 };
    //             b[i].y = 1.0;
    //             b[i].z = if b[i].z >= 1.0 { 0.0 } else { b[i].z + 0.01 };
    //             b[i].w = 1.0;

    //             let i = 2;
    //             b[i].x = if b[i].x >= 1.0 { 0.0 } else { b[i].x + 0.01 };
    //             b[i].y = if b[i].y >= 1.0 { 0.0 } else { b[i].y + 0.01 };
    //             b[i].z = 1.0;
    //             b[i].w = 1.0;
    //         }
    //         */

    //         WHBGfxBeginRender();

    //         // println!("begin render");

    //         WHBGfxBeginRenderTV();
    //         WHBGfxClearColor(0.0, 0.0, 1.0, 1.0);
    //         // GX2RSetAttributeBuffer(
    //         //     position_buffer.as_raw_mut(),
    //         //     0,
    //         //     core::mem::size_of::<Float4>() as u32,
    //         //     0,
    //         // );
    //         // GX2RSetAttributeBuffer(
    //         //     color_buffer.as_raw_mut(),
    //         //     1,
    //         //     core::mem::size_of::<Float4>() as u32,
    //         //     0,
    //         // );
    //         // GX2SetFetchShader(&group.fetchShader);
    //         // GX2SetVertexShader(group.vertexShader);
    //         // GX2SetPixelShader(group.pixelShader);
    //         // GX2DrawEx(GX2PrimitiveMode::GX2_PRIMITIVE_MODE_TRIANGLES, 3, 0, 1);
    //         WHBGfxFinishRenderTV();

    //         WHBGfxBeginRenderDRC();
    //         WHBGfxClearColor(1.0, 0.0, 1.0, 1.0);
    //         // GX2RSetAttributeBuffer(
    //         //     position_buffer.as_raw_mut(),
    //         //     0,
    //         //     core::mem::size_of::<Float4>() as u32,
    //         //     0,
    //         // );
    //         // GX2RSetAttributeBuffer(
    //         //     color_buffer.as_raw_mut(),
    //         //     1,
    //         //     core::mem::size_of::<Float4>() as u32,
    //         //     0,
    //         // );
    //         // GX2SetFetchShader(&group.fetchShader);
    //         // GX2SetVertexShader(group.vertexShader);
    //         // GX2SetPixelShader(group.pixelShader);
    //         // GX2DrawEx(GX2PrimitiveMode::GX2_PRIMITIVE_MODE_TRIANGLES, 3, 0, 1);
    //         WHBGfxFinishRenderDRC();

    //         WHBGfxFinishRender();
    //     }
    // }
}
