use embla::ecs::World;
use failure::Error;

use components::{ColoredCircle, ColoredRect, Position};
use render_interface::RenderInterface;

pub fn render_primitives(world: &mut World, renderer: &mut RenderInterface) -> Result<(), Error> {
    for (position, r) in world.with_components::<(Position, ColoredRect)>() {
        let pos = position.0;
        let rect = (
            r.rect.0 as f32 + pos.0,
            r.rect.1 as f32 + pos.1,
            r.rect.2 as f32 + pos.0,
            r.rect.2 as f32 + pos.1,
        );
        renderer.draw_rect(rect, r.color)?;
    }

    for (position, c) in world.with_components::<(Position, ColoredCircle)>() {
        renderer.draw_circle(position.0, c.radius, 20, c.color)?;
    }

    Ok(())
}
