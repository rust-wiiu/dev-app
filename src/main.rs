#![no_std]
#![no_main]

use cafe_app::Application;
use cafe_rs::rt::rpx;
use cafe_rs::sys::gx2;
use cafe_rs::{self as cafe, sys};

rpx! {
    fn main() {
        cafe_logger::CafeLogger::new().level(log::Level::Debug).init().unwrap();

        MyApp::run().unwrap()
    }
}

static SHADER: &[u8] = include_bytes!("../content/pos_col_shader.gsh");

type Position = [f32; 4];
type Color = [f32; 4];

struct MyApp {
    shader: cafe::graphics::ShaderGroup<(Position, Color)>,
    position: cafe::graphics::VertexBuffer<Position>,
    color: cafe::graphics::VertexBuffer<Color>,
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
            (
                cafe::graphics::Attribute::<Color>::location(0),
                cafe::graphics::Attribute::<Position>::location(1),
            ),
        );

        Self {
            shader: group,
            position: cafe::graphics::VertexBuffer::from([
                [1.0, -1.0, 0.0, 1.0],  //
                [0.0, 1.0, 0.0, 1.0],   //
                [-1.0, -1.0, 1.0, 1.0], //
            ]),
            color: cafe::graphics::VertexBuffer::from([
                [1.0, 0.0, 0.0, 1.0], //
                [0.0, 1.0, 0.0, 1.0], //
                [0.0, 0.0, 1.0, 1.0], //
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
            let mut data = self.color.lock();

            {
                let col = &mut data[0];

                col[1] = if col[1] >= 1.0 { 0.0 } else { col[1] + 0.01 };
                col[2] = if col[2] >= 1.0 { 0.0 } else { col[2] + 0.01 };
            }

            {
                let col = &mut data[1];

                col[0] = if col[0] >= 1.0 { 0.0 } else { col[0] + 0.01 };
                col[2] = if col[2] >= 1.0 { 0.0 } else { col[2] + 0.01 };
            }

            {
                let col = &mut data[2];

                col[0] = if col[0] >= 1.0 { 0.0 } else { col[0] + 0.01 };
                col[1] = if col[1] >= 1.0 { 0.0 } else { col[1] + 0.01 };
            }
        }

        // log::info!("{:?}", data);
    }

    fn render(&mut self) {
        self.tv_ctx.direct_render(
            self.tv_target.as_mut().unwrap(),
            sys::gx2::state::RenderTarget::T0,
            |p| {
                p.set_color((0, 0, 255));
                p.use_shader_group(&self.shader, (&self.color, &self.position));
                p.draw(
                    sys::gx2::shader::PrimitiveMode::Triangles,
                    self.position.len(),
                    1,
                );
            },
        );

        self.drc_ctx.direct_render(
            self.drc_target.as_mut().unwrap(),
            sys::gx2::state::RenderTarget::T0,
            |p| {
                p.set_color((0, 0, 255));
                p.use_shader_group(&self.shader, (&self.color, &self.position));
                p.draw(
                    sys::gx2::shader::PrimitiveMode::Triangles,
                    self.position.len(),
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
        // self.drc = None;
        self.tv_target = None;
        // self.drc_target = None;
    }
}
