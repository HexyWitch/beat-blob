use failure::Error;

use embla::math::Vec2;

use embla::graphics::TextureImage;

pub trait RenderInterface {
    fn screen_size(&self) -> (i32, i32);

    fn draw_texture(
        &mut self,
        texture: &TextureImage,
        position: Vec2,
        scale: f32,
        rotation: f32,
    ) -> Result<(), Error>;

    fn draw_rect(
        &mut self,
        rect: (f32, f32, f32, f32),
        color: (f32, f32, f32, f32),
    ) -> Result<(), Error>;

    fn draw_circle(
        &mut self,
        center: Vec2,
        radius: f32,
        points: i32,
        color: (f32, f32, f32, f32),
    ) -> Result<(), Error>;
}
