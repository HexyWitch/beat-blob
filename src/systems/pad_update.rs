use embla::ecs::World;
use embla::input::{Input, Key};
use failure::Error;

use components::{ColoredCircle, Pad, PadTeam, TilePosition, TileTrigger};
use grid::Grid;

const PAD_PULSE_TIME: f32 = 0.1;
pub fn pad_update(dt: f32, input: &Input, grid: &Grid, world: &mut World) -> Result<(), Error> {
    let mut triggered_tiles = Vec::new();
    for (mut circle, mut pad, team, tile_pos) in
        world.with_components::<(ColoredCircle, Pad, PadTeam, TilePosition)>()
    {
        pad.pulse_timer = (pad.pulse_timer - dt).max(0.0);

        let mut trigger = |pad: &mut Pad| {
            triggered_tiles.push(*tile_pos);
            pad.pulse_timer = PAD_PULSE_TIME;
        };

        match *team {
            PadTeam::Blue if input.key_is_pressed(&Key::A) => trigger(&mut pad),
            PadTeam::Red if input.key_is_pressed(&Key::S) => trigger(&mut pad),
            PadTeam::Green if input.key_is_pressed(&Key::D) => trigger(&mut pad),
            PadTeam::Yellow if input.key_is_pressed(&Key::F) => trigger(&mut pad),
            _ => {}
        }

        let max_size = grid.cell_height() as f32 * 0.45;
        let min_size = grid.cell_height() as f32 * 0.2;
        let r = pad.pulse_timer / PAD_PULSE_TIME;
        circle.radius = min_size + (r * (max_size - min_size));
    }

    for tile in triggered_tiles {
        world.add_entity().insert(TileTrigger).insert(tile);
    }

    Ok(())
}
