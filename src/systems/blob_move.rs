use embla::ecs::{EntityId, World};
use embla::math::Vec2;
use failure::Error;

use components::{Blob, Position, TilePosition};
use grid::Grid;

pub fn tween_blobs(beat_timer: f32, grid: &Grid, world: &mut World) -> Result<(), Error> {
    for (mut position, tile_pos, _) in world.with_components::<(Position, TilePosition, Blob)>() {
        let height = grid.cell_height() as f32;
        let width = grid.cell_width() as f32;
        let cell = grid.cell_rect(tile_pos.0, tile_pos.1);
        let center = Vec2::new(cell.0 as f32 + width / 2.0, cell.1 as f32 + height / 2.0);
        position.0 = center + Vec2::new(0.0, -height * beat_timer)
    }

    Ok(())
}

pub fn move_blobs(world: &mut World) -> Result<(), Error> {
    let mut removed = Vec::new();
    for (e, mut tile_pos, _) in world.with_components::<(EntityId, TilePosition, Blob)>() {
        tile_pos.1 -= 1;
        if tile_pos.1 < 0 {
            removed.push(*e);
        }
    }
    for e in removed {
        world.remove_entity(e.0);
    }
    Ok(())
}
