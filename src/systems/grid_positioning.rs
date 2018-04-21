use embla::ecs::World;
use embla::math::Vec2;
use failure::Error;

use components::{Position, TilePosition};
use grid::Grid;

pub fn grid_positioning(grid: &Grid, world: &mut World) -> Result<(), Error> {
    for (tile_pos, mut position) in world.with_components::<(TilePosition, Position)>() {
        let r = grid.cell_rect(tile_pos.0, tile_pos.1);
        let center = Vec2::new(
            grid.cell_width() as f32 * 0.5,
            grid.cell_height() as f32 * 0.5,
        );
        position.0 = Vec2::new(r.0 as f32, r.1 as f32) + center;
    }

    Ok(())
}
