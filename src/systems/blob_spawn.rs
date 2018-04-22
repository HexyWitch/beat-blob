use embla::ecs::World;
use embla::math::Vec2;
use failure::Error;

use components::{Blob, BlobGoal, BlobSpawn, ColoredCircle, FillMode, PadTeam, Position,
                 TilePosition};
use grid::Grid;

pub fn spawn_blobs(grid: &Grid, world: &mut World) -> Result<(), Error> {
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
        let goal_pos = world
            .with_components::<(TilePosition, PadTeam, BlobGoal)>()
            .filter(|(_, t, _)| **t == team)
            .next()
            .map(|(pos, _, _)| (pos.0, pos.1))
            .unwrap();
        let path = grid.find_path((x, y), goal_pos).unwrap();
        create_blob(world, x, y, team, path)?;
    }
    Ok(())
}

const BLOB_RADIUS: f32 = 15.0;
fn create_blob(
    world: &mut World,
    x: i32,
    y: i32,
    team: PadTeam,
    path: Vec<(i32, i32)>,
) -> Result<(), Error> {
    world
        .add_entity()
        .insert(Position(Vec2::zero()))
        .insert(TilePosition(x, y))
        .insert(ColoredCircle {
            radius: BLOB_RADIUS,
            color: team.color(),
            fill: FillMode::Filled,
        })
        .insert(team)
        .insert(Blob {
            path_index: 0,
            path,
        });

    Ok(())
}
