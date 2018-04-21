use std;

use embla::ecs::World;
use embla::math::Vec2;
use failure::Error;

use components::{ColoredCircle, ColoredRect, FillMode, Position};
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
        if c.fill == FillMode::Filled {
            renderer.draw_circle(position.0, c.radius, 20, c.color)?;
        } else if let FillMode::Outline(width) = c.fill {
            let num_points = 20;
            let points = (0..num_points)
                .map(|i| {
                    let a = i as f32 * (std::f32::consts::PI * 2.0 / num_points as f32);
                    position.0 + Vec2::with_angle(a) * c.radius
                })
                .collect::<Vec<_>>();
            let mut other_points = points.iter().skip(1).cloned().collect::<Vec<_>>();
            other_points.push(points[0]);
            for (p1, p2) in points.into_iter().zip(other_points.into_iter()) {
                renderer.draw_line(p1, p2, width, c.color)?;
            }
        }
    }

    Ok(())
}
