use embla::ecs::{EntityId, World};
use failure::Error;

use components::{Blob, TilePosition};

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
