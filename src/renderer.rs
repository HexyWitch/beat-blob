use failure::Error;
use std;
use std::rc::Rc;

use embla::assets::Image;
use embla::graphics::{TextureAtlas, TextureImage};
use embla::math::Vec2;
use embla::rendering_api::{Program, Renderer, Texture, TextureFiltering, Uniform, Vertex,
                           VertexAttributeType};

use render_interface::RenderInterface;

static VERTEX_SHADER: &'static str = include_str!("../shaders/vertex.glsl");
static FRAGMENT_SHADER: &'static str = include_str!("../shaders/fragment.glsl");

pub struct TexturedVertex {
    pub position: (f32, f32),
    pub tex_coord: (f32, f32),
    pub color: (f32, f32, f32, f32),
}

impl Vertex for TexturedVertex {
    fn attributes() -> Vec<(String, usize, VertexAttributeType)> {
        vec![
            ("position".into(), 2, VertexAttributeType::Float),
            ("tex_coord".into(), 2, VertexAttributeType::Float),
            ("color".into(), 4, VertexAttributeType::Float),
        ]
    }
}

pub struct GameRenderer<R: Renderer> {
    program: R::Program,
    vertex_buffer: R::VertexBuffer,
    vertices: Vec<TexturedVertex>,
    texture: Rc<R::Texture>,
    atlas: TextureAtlas,
    white_texture: [u32; 4],
}

impl<R> GameRenderer<R>
where
    R: Renderer,
{
    pub fn new() -> Result<GameRenderer<R>, Error>
    where
        R: Renderer,
    {
        let mut program = R::create_program(VERTEX_SHADER, FRAGMENT_SHADER)?;

        let texture_size = (4096, 4096);

        let texture = Rc::new(R::create_texture(
            texture_size,
            Some(TextureFiltering::Nearest),
        )?);

        let screen_size = R::screen_size();
        program.set_uniform(
            "screen_size",
            Uniform::Vec2((screen_size.0 as f32, screen_size.1 as f32)),
        );
        program.set_uniform(
            "texture_size",
            Uniform::Vec2((texture_size.0 as f32, texture_size.1 as f32)),
        );
        program.set_uniform("texture", Uniform::Texture(texture.clone()));

        let mut atlas = TextureAtlas::new(texture_size);
        let white_image = TextureImage::new(Rc::new(Image {
            data: vec![255, 255, 255, 255],
            width: 1,
            height: 1,
        }));
        let white_texture = atlas.add_texture(&white_image)?;
        texture.set_region(white_image.image(), (white_texture[0], white_texture[1]));

        Ok(GameRenderer::<R> {
            program,
            vertex_buffer: R::create_vertex_buffer()?,
            vertices: Vec::new(),
            texture,
            atlas,
            white_texture,
        })
    }

    pub fn do_render(&mut self) -> Result<(), Error> {
        R::clear(Some((0.0, 0.0, 0.0, 1.0)));

        R::render_vertices(&self.vertex_buffer, &self.program, &self.vertices)?;

        self.vertices.clear();

        Ok(())
    }
}

impl<R> RenderInterface for GameRenderer<R>
where
    R: Renderer,
{
    fn screen_size(&self) -> (i32, i32) {
        R::screen_size()
    }

    fn draw_texture(
        &mut self,
        texture: &TextureImage,
        position: Vec2,
        scale: f32,
        rotation: f32,
    ) -> Result<(), Error> {
        let tex_region = match self.atlas.get_texture_block(texture) {
            Some(region) => region,
            None => {
                let region = self.atlas.add_texture(texture)?;
                self.texture
                    .set_region(texture.image(), (region[0], region[1]));
                region
            }
        };
        let size = (tex_region[2] - tex_region[0], tex_region[3] - tex_region[1]);

        let rect = (
            size.0 as f32 / -2.0 * scale,
            size.1 as f32 / -2.0 * scale,
            size.0 as f32 / 2.0 * scale,
            size.1 as f32 / 2.0 * scale,
        );

        let rotate = |(x, y), a: f32| (x * a.cos() - y * a.sin(), x * a.sin() + y * a.cos());
        let quad = [
            rotate((rect.0, rect.1), rotation),
            rotate((rect.0, rect.3), rotation),
            rotate((rect.2, rect.3), rotation),
            rotate((rect.2, rect.1), rotation),
        ];

        let ll = (position.0 + quad[0].0, position.1 + quad[0].1);
        let ul = (position.0 + quad[1].0, position.1 + quad[1].1);
        let ur = (position.0 + quad[2].0, position.1 + quad[2].1);
        let lr = (position.0 + quad[3].0, position.1 + quad[3].1);
        let verts = [
            (ll, (tex_region[0], tex_region[1])),
            (ul, (tex_region[0], tex_region[3])),
            (lr, (tex_region[2], tex_region[1])),
            (ul, (tex_region[0], tex_region[3])),
            (ur, (tex_region[2], tex_region[3])),
            (lr, (tex_region[2], tex_region[1])),
        ];
        for &(pos, tex_coord) in verts.iter() {
            self.vertices.push(TexturedVertex {
                position: pos,
                tex_coord: (tex_coord.0 as f32, tex_coord.1 as f32),
                color: (1.0, 1.0, 1.0, 1.0),
            })
        }

        Ok(())
    }

    fn draw_rect(
        &mut self,
        rect: (f32, f32, f32, f32),
        color: (f32, f32, f32, f32),
    ) -> Result<(), Error> {
        let ll = (rect.0, rect.1);
        let ul = (rect.0, rect.3);
        let ur = (rect.2, rect.3);
        let lr = (rect.2, rect.1);
        let verts = [
            (ll, (self.white_texture[0], self.white_texture[1])),
            (ul, (self.white_texture[0], self.white_texture[3])),
            (lr, (self.white_texture[2], self.white_texture[1])),
            (ul, (self.white_texture[0], self.white_texture[3])),
            (ur, (self.white_texture[2], self.white_texture[3])),
            (lr, (self.white_texture[2], self.white_texture[1])),
        ];
        for &(pos, tex_coord) in verts.iter() {
            self.vertices.push(TexturedVertex {
                position: pos,
                tex_coord: (tex_coord.0 as f32, tex_coord.1 as f32),
                color: color,
            })
        }

        Ok(())
    }

    fn draw_circle(
        &mut self,
        center: Vec2,
        radius: f32,
        points: i32,
        color: (f32, f32, f32, f32),
    ) -> Result<(), Error> {
        let mut verts = Vec::new();

        let points = (0..points)
            .map(|i| {
                let a = i as f32 * (std::f32::consts::PI * 2.0 / points as f32);
                center + Vec2::with_angle(a) * radius
            })
            .collect::<Vec<_>>();
        let mut other_points = points.iter().skip(1).cloned().collect::<Vec<_>>();
        other_points.push(points[0]);
        for (p1, p2) in points.into_iter().zip(other_points.into_iter()) {
            verts.extend_from_slice(&[
                (
                    (center.0, center.1),
                    (self.white_texture[0], self.white_texture[1]),
                ),
                ((p1.0, p1.1), (self.white_texture[2], self.white_texture[1])),
                ((p2.0, p2.1), (self.white_texture[2], self.white_texture[3])),
            ])
        }

        for &(pos, tex_coord) in verts.iter() {
            self.vertices.push(TexturedVertex {
                position: pos,
                tex_coord: (tex_coord.0 as f32, tex_coord.1 as f32),
                color: color,
            })
        }

        Ok(())
    }

    fn draw_line(
        &mut self,
        p1: Vec2,
        p2: Vec2,
        width: f32,
        color: (f32, f32, f32, f32),
    ) -> Result<(), Error> {
        let rotate = |v: Vec2, a: f32| {
            Vec2::new(v.0 * a.cos() - v.1 * a.sin(), v.0 * a.sin() + v.1 * a.cos())
        };

        let length = (p2 - p1).mag();
        let a = (p2 - p1).angle();
        let ll = rotate(Vec2::new(0.0, -width / 2.0), a) + p1;
        let ul = rotate(Vec2::new(0.0, width / 2.0), a) + p1;
        let ur = rotate(Vec2::new(length, width / 2.0), a) + p1;
        let lr = rotate(Vec2::new(length, -width / 2.0), a) + p1;
        let verts = [
            (ll, (self.white_texture[0], self.white_texture[1])),
            (ul, (self.white_texture[0], self.white_texture[3])),
            (lr, (self.white_texture[2], self.white_texture[1])),
            (ul, (self.white_texture[0], self.white_texture[3])),
            (ur, (self.white_texture[2], self.white_texture[3])),
            (lr, (self.white_texture[2], self.white_texture[1])),
        ];

        for &(pos, tex_coord) in verts.iter() {
            self.vertices.push(TexturedVertex {
                position: (pos.0, pos.1),
                tex_coord: (tex_coord.0 as f32, tex_coord.1 as f32),
                color: color,
            })
        }

        Ok(())
    }
}
