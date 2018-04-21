use embla::ecs::World;
use embla::math::Vec2;
use failure::Error;

use components::{Blob, BlobSpawn, ColoredCircle, PadTeam, Position, TilePosition};

pub fn spawn_blobs(world: &mut World) -> Result<(), Error> {
    let mut spawns = Vec::new();
    for (tile_pos, team, mut spawner) in
        world.with_components::<(TilePosition, PadTeam, BlobSpawn)>()
    {
        spawner.timer -= 1;
        if spawner.timer == 0 {
            spawns.push((tile_pos.0, tile_pos.1, *team));
            spawner.timer = spawner.interval;
        }
    }
    for (x, y, team) in spawns {
        create_blob(world, x, y, team)?;
    }
    Ok(())
}

const BLOB_RADIUS: f32 = 15.0;
fn create_blob(world: &mut World, x: i32, y: i32, team: PadTeam) -> Result<(), Error> {
    world
        .add_entity()
        .insert(Position(Vec2::zero()))
        .insert(TilePosition(x, y))
        .insert(ColoredCircle {
            radius: BLOB_RADIUS,
            color: team.color(),
        })
        .insert(team)
        .insert(Blob);

    Ok(())
}
