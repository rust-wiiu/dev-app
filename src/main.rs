#![no_std]
#![no_main]

use cafe_app::Application;
use cafe_rs::prelude::*;
use cafe_rs::rt::rpx;

rpx! {
    fn main() {
        cafe_logger::CafeLogger::new().level(log::Level::Debug).init().unwrap();

        MyApp::run().unwrap()
    }
}

static SHADER: &[u8] = include_bytes!("../content/pos_col_shader.gsh");

type Position = [f32; 4];
type Color = [f32; 4];

struct Vertex {
    position: [f32; 4],
    color: [f32; 4],
}

// struct Attributes([cafe::graphics::Attribute; 2]);

// impl Attributes {
//     fn new() -> Self {
//         todo!()
//     }

//     fn color(&self) -> &cafe::graphics::Attribute {
//         &self.0[0]
//     }

//     fn position(&self) -> &cafe::graphics::Attribute {
//         &self.0[1]
//     }
// }

// impl AsRef<[cafe::graphics::Attribute]> for Attributes {
//     fn as_ref(&self) -> &[cafe::graphics::Attribute] {
//         &self.0
//     }
// }

struct MyApp {
    shader: cafe::graphics::ShaderGroup<[cafe::graphics::Attribute; 2]>,
    // position: cafe::graphics::VertexBuffer<Position>,
    // color: cafe::graphics::VertexBuffer<Color>,
    vbo: cafe::graphics::VertexBuffer<Vertex>,
    tv_ctx: cafe::graphics::Context<cafe::graphics::TV>,
    drc_ctx: cafe::graphics::Context<cafe::graphics::DRC>,
    tv: Option<cafe::graphics::Display<cafe::graphics::TV>>,
    drc: Option<cafe::graphics::Display<cafe::graphics::DRC>>,
    tv_target: Option<cafe::graphics::Target<cafe::graphics::TV>>,
    drc_target: Option<cafe::graphics::Target<cafe::graphics::DRC>>,
}

impl Application for MyApp {
    fn new() -> Self {
        let mut gfx = cafe::graphics::Gfx2::parse(SHADER).unwrap();

        let group = cafe::graphics::ShaderGroup::new(
            gfx.vertex.swap_remove(0),
            gfx.pixel.swap_remove(0),
            [
                cafe::graphics::Attribute::new::<Color>(0, cafe::graphics::shader::Stream::S0)
                    .offset(std::mem::offset_of!(Vertex, color)),
                cafe::graphics::Attribute::new::<Position>(1, cafe::graphics::shader::Stream::S0)
                    .offset(std::mem::offset_of!(Vertex, position)),
            ],
        );

        Self {
            shader: group,
            // position: cafe::graphics::VertexBuffer::from([
            //     [1.0, -1.0, 0.0, 1.0],  //
            //     [0.0, 1.0, 0.0, 1.0],   //
            //     [-1.0, -1.0, 1.0, 1.0], //
            // ]),
            // color: cafe::graphics::VertexBuffer::from([
            //     [1.0, 0.0, 0.0, 1.0], //
            //     [0.0, 1.0, 0.0, 1.0], //
            //     [0.0, 0.0, 1.0, 1.0], //
            // ]),
            vbo: cafe::graphics::VertexBuffer::from([
                Vertex {
                    position: [1.0, -1.0, 0.0, 1.0],
                    color: [1.0, 0.0, 0.0, 1.0],
                },
                Vertex {
                    position: [0.0, 1.0, 0.0, 1.0],
                    color: [0.0, 1.0, 0.0, 1.0],
                },
                Vertex {
                    position: [-1.0, -1.0, 1.0, 1.0],
                    color: [0.0, 0.0, 1.0, 1.0],
                },
            ]),
            tv: None,
            drc: None,
            tv_ctx: cafe::graphics::Context::new(),
            drc_ctx: cafe::graphics::Context::new(),
            tv_target: None,
            drc_target: None,
        }
    }

    fn update(&mut self) {
        {
            let mut data = self.vbo.lock();

            {
                let v = &mut data[0];

                v.color[1] = (v.color[1] + 0.01) % 1.0;
                v.color[2] = (v.color[2] + 0.01) % 1.0;
            }

            {
                let v = &mut data[1];

                v.color[0] = (v.color[0] + 0.01) % 1.0;
                v.color[2] = (v.color[2] + 0.01) % 1.0;
            }

            {
                let v = &mut data[2];

                v.color[0] = (v.color[0] + 0.01) % 1.0;
                v.color[1] = (v.color[1] + 0.01) % 1.0;
            }
        }
    }

    fn render(&mut self) {
        self.tv_ctx.direct_render(
            self.tv_target.as_mut().unwrap(),
            sys::gx2::state::RenderTarget::T0,
            |p| {
                p.set_color((0, 0, 255));
                p.use_shader_group(&self.shader);
                p.set_attribute_stream(&self.shader.attrs[0], &self.vbo);
                p.set_attribute_stream(&self.shader.attrs[1], &self.vbo);
                p.draw(
                    sys::gx2::shader::PrimitiveMode::Triangles,
                    self.vbo.len(),
                    1,
                );
            },
        );

        self.drc_ctx.direct_render(
            self.drc_target.as_mut().unwrap(),
            sys::gx2::state::RenderTarget::T0,
            |p| {
                p.set_color((0, 255, 0));
                p.use_shader_group(&self.shader);
                p.set_attribute_stream(&self.shader.attrs[0], &self.vbo);
                p.set_attribute_stream(&self.shader.attrs[1], &self.vbo);
                p.draw(
                    sys::gx2::shader::PrimitiveMode::Triangles,
                    self.vbo.len(),
                    1,
                );
            },
        );
    }

    fn on_acquire(&mut self) {
        self.tv = Some(cafe::graphics::Display::tv());
        self.drc = Some(cafe::graphics::Display::drc());
        self.tv_target = Some(cafe::graphics::Target::tv());
        self.drc_target = Some(cafe::graphics::Target::drc());
    }

    fn on_release(&mut self) {
        self.tv = None;
        self.drc = None;
        self.tv_target = None;
        self.drc_target = None;
    }
}
