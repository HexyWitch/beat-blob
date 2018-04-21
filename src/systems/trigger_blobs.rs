use embla::ecs::{EntityId, World};
use failure::Error;

use components::{Blob, TilePosition, TileTrigger};

pub fn trigger_blobs(world: &mut World) -> Result<(), Error> {
    let (trigger_entities, triggered_tiles): (Vec<usize>, Vec<TilePosition>) = world
        .with_components::<(EntityId, TileTrigger, TilePosition)>()
        .map(|(e, _, t)| (e.0, *t))
        .unzip();
    for e in trigger_entities {
        world.remove_entity(e);
    }

    let remove: Vec<usize> = world
        .with_components::<(EntityId, Blob, TilePosition)>()
        .filter_map(|(id, _, tile_pos)| {
            if triggered_tiles.iter().any(|t| *t == *tile_pos) {
                Some(id.0)
            } else {
                None
            }
        })
        .collect();
    for id in remove {
        world.remove_entity(id);
    }

    Ok(())
}
