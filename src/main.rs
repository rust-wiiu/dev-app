#![no_std]
#![no_main]
#![allow(unused_imports)]

use wut::font::icons::keyboard;
use wut::gx2::dialog::ErrorView;
use wut::gx2::dialog::keyboard::Keyboard;
use wut::gx2::shader::Attribute;
use wut::gx2::target::{RenderTarget, Renderable, TV};
use wut::gx2::{
    dialog::Dialog,
    shader::{self, PrimitiveMode},
    // types::{Float4, Uint, Uint4},
};
use wut::prelude::*;
use wut::screen::Color;
use wut::*;

use guii::Guii;

// use cafeglsl;

// #[derive(ShaderAttributes)]
// struct MyShader {
//     #[name = "in_position"]
//     position: shader::Attribute<Float4>,
//     #[name = "in_color"]
//     color: shader::Attribute<Float4>,
// }

// static PROGRAM: shader::Program = shader::Program::from(include_bytes!("out.gsh"));

#[main(Cafe)]
fn main() {
    // let mut shader: shader::Shader = shader::Shader::new(
    //     0,
    //     &PROGRAM,
    //     [
    //         Attribute::new::<Float4>("in_position", 0, 0),
    //         Attribute::new::<Float4>("in_color", 1, 0),
    //     ],
    // )
    // .unwrap();

    // let mut vertices = gx2::buffer::VertexBuffer::new()
    //     .cpu(true, true)
    //     .gpu(true, false)
    //     .from([
    //         Float4::from((0.5, 0.5, 0.0, 1.0)),
    //         Float4::from((0.5, -0.5, 0.0, 1.0)),
    //         Float4::from((-0.5, -0.5, 1.0, 1.0)),
    //         Float4::from((-0.5, 0.5, 0.0, 1.0)),
    //     ])
    //     .unwrap();

    // let mut colors = gx2::buffer::VertexBuffer::new()
    //     .cpu(true, true)
    //     .gpu(true, false)
    //     .from([
    //         Float4::from(gx2::color::Color::red()),
    //         Float4::from(gx2::color::Color::green()),
    //         Float4::from(gx2::color::Color::blue()),
    //         Float4::from(gx2::color::Color::magenta()),
    //         // Float4::from(gx2::color::Color::black()),
    //     ])
    //     .unwrap();

    // let indices = gx2::buffer::IndexBuffer::new()
    //     .cpu(true, true)
    //     .gpu(true, false)
    //     .from([0u32, 1, 2, 0, 2, 3])
    //     .unwrap();

    let context = gx2::context::Context::new();

    // let mut delta = 0.0;

    // let error = ErrorView::build(gx2::dialog::Region::Europe, gx2::dialog::Language::English)
    //     .error_code(2)
    //     .show()
    //     .unwrap();

    // wut::thread::sleep(wut::time::Duration::from_secs(5));

    let mut guii = Guii::new().unwrap();

    while process::running() {
        // delta += 0.05;

        {
            // let mut b = color_buffer.lock().unwrap();
            // let i = 0;
            // b[i].x = 0.0;
            // b[i].y = 0.0;
            // b[i].z = 0.0;

            // let i = 1;
            // b[i].x = 1.0;
            // b[i].y = 0.5 * (1.0 + delta.cos());
            // b[i].z = 0.5 * (1.0 + delta.cos());
            // b[i].w = 0.5 * (1.0 + delta.cos());

            // let i = 2;
            // b[i].x = 1.0;
            // b[i].y = 1.0;
            // b[i].z = 1.0;
            // b[i].w = 1.0;
        }

        guii.build(|ui| {
            ui.rect(100, 100, 100, 100, Color::red());

            ui.text(
                " !\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~",
                100,
                100,
                0.5,
                Color::black(),
            );

            ui.rect(100, 100, 90, 90, Color::green());
        });

        context.render(
            |tv| {
                tv.fill(gx2::color::Color::light_blue());

                guii.render(tv);

                // shader
                //     .render(tv)
                //     .attribute(&vertices)
                //     .attribute(&colors)
                //     // .draw();
                //     .draw_indexed(&indices);

                // error.update();
                // error.render(tv);
            },
            |drc| {
                drc.fill(gx2::color::Color::dark_olive_green());
            },
        );
    }
}
