use embla::ecs::{EntityId, World};
use embla::math::Vec2;
use failure::Error;

use components::{Blob, Position, TilePosition};
use grid::Grid;

pub fn tween_blobs(beat_timer: f32, grid: &Grid, world: &mut World) -> Result<(), Error> {
    for (mut position, _, blob) in world.with_components::<(Position, TilePosition, Blob)>() {
        let center_offset = Vec2::new(
            grid.cell_width() as f32 * 0.5,
            grid.cell_height() as f32 * 0.5,
        );
        let tile_position = |t: (i32, i32)| {
            let r = grid.cell_rect(t.0, t.1);
            Vec2::new(r.0 as f32, r.1 as f32)
        };

        let (timer, from, to) = if beat_timer <= 0.5 {
            let prev = tile_position(blob.path[(blob.path_index.max(1) - 1)]);
            let next = tile_position(blob.path[blob.path_index]);
            (beat_timer / 0.5, prev + (next - prev) * 0.5, next)
        } else {
            let prev = tile_position(blob.path[blob.path_index]);
            let next = tile_position(blob.path[(blob.path_index + 1).min(blob.path.len() - 1)]);
            ((beat_timer - 0.5) / 0.5, prev, next - (next - prev) * 0.5)
        };

        position.0 = (from + (to - from) * timer) + center_offset;
    }

    Ok(())
}

pub fn move_blobs(world: &mut World) -> Result<(), Error> {
    let mut removed = Vec::new();
    for (e, mut tile_pos, mut blob) in world.with_components::<(EntityId, TilePosition, Blob)>() {
        blob.path_index += 1;
        if blob.path_index >= blob.path.len() {
            removed.push(e.0);
        } else {
            let pos = blob.path[blob.path_index];
            *tile_pos = TilePosition(pos.0, pos.1);
        }
    }
    for e in removed {
        world.remove_entity(e);
    }
    Ok(())
}
