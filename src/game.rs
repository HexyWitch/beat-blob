use embla::ecs::World;
use embla::input::{Input, Key, MouseButton};
use embla::math::Vec2;
use failure::Error;

use grid::Grid;
use render_interface::RenderInterface;

use components::{Blob, ColoredCircle, ColoredRect, Pad, PadTeam, Position, TilePosition};

const PAD_PULSE_TIME: f32 = 0.1;

pub struct Game {
    grid: Grid,
    hovered_tile: Option<(i32, i32)>,
    screen_size: Vec2,
    world: World,
}

impl Game {
    pub fn new() -> Result<Game, Error> {
        let mut game = Game {
            grid: Grid::new(6, 10, 40, 40),
            hovered_tile: None,
            screen_size: Vec2::new(0.0, 0.0),
            world: World::new(),
        };

        game.init()?;

        Ok(game)
    }

    fn init(&mut self) -> Result<(), Error> {
        self.insert_pad(1, 1, PadTeam::Blue)?;
        self.insert_pad(2, 1, PadTeam::Red)?;
        self.insert_pad(3, 1, PadTeam::Green)?;
        self.insert_pad(4, 1, PadTeam::Yellow)?;

        Ok(())
    }

    fn insert_pad(&mut self, x: i32, y: i32, team: PadTeam) -> Result<(), Error> {
        self.world
            .add_entity()
            .insert(Position(Vec2::zero()))
            .insert(TilePosition(x, y))
            .insert(ColoredCircle {
                radius: self.grid.cell_width() as f32 * 0.2,
                color: team.color(),
            })
            .insert(team)
            .insert(Pad {
                triggered: false,
                pulse_timer: 0.0,
            });

        Ok(())
    }

    pub fn update(&mut self, dt: f32, input: &Input) -> Result<(), Error> {
        let mut mouse_position = input.mouse_position();
        mouse_position.1 = self.screen_size.1 as f32 - mouse_position.1;
        self.hovered_tile = self.grid.tile_at(mouse_position);

        if let Some(tile) = self.hovered_tile {
            if input.mouse_button_is_down(&MouseButton::Left) {
                let size = (self.grid.cell_width(), self.grid.cell_height());
                let rect = (1.0, 1.0, size.0 as f32 - 1.0, size.1 as f32 - 1.0);
                self.world
                    .add_entity()
                    .insert(Position(Vec2::zero()))
                    .insert(TilePosition(tile.0, tile.1))
                    .insert(ColoredRect {
                        rect,
                        color: (1.0, 1.0, 1.0, 1.0),
                    });
            }
        }

        for (tile_pos, mut position) in self.world.with_components::<(TilePosition, Position)>() {
            let r = self.grid.cell_rect(tile_pos.0, tile_pos.1);
            position.0 = Vec2::new(r.0 as f32, r.1 as f32);
        }

        for (mut circle, mut pad, team) in self.world
            .with_components::<(ColoredCircle, Pad, PadTeam)>()
        {
            pad.triggered = false;
            pad.pulse_timer = (pad.pulse_timer - dt).max(0.0);

            let trigger = |pad: &mut Pad| {
                pad.triggered = true;
                pad.pulse_timer = PAD_PULSE_TIME;
            };

            match *team {
                PadTeam::Blue if input.key_is_pressed(&Key::A) => trigger(&mut pad),
                PadTeam::Red if input.key_is_pressed(&Key::S) => trigger(&mut pad),
                PadTeam::Green if input.key_is_pressed(&Key::D) => trigger(&mut pad),
                PadTeam::Yellow if input.key_is_pressed(&Key::F) => trigger(&mut pad),
                _ => {}
            }

            let max_size = self.grid.cell_height() as f32 * 0.45;
            let min_size = self.grid.cell_height() as f32 * 0.2;
            let r = pad.pulse_timer / PAD_PULSE_TIME;
            circle.radius = min_size + (r * (max_size - min_size));
        }

        Ok(())
    }

    pub fn render(&mut self, renderer: &mut RenderInterface) -> Result<(), Error> {
        let screen_size = renderer.screen_size();
        self.screen_size = Vec2::new(screen_size.0 as f32, screen_size.1 as f32);

        // render grid
        for x in 0..self.grid.width() {
            for y in 0..self.grid.height() {
                let r = self.grid.cell_rect(x, y);
                let r = (r.0 as f32, r.1 as f32, r.2 as f32, r.3 as f32);

                let color = if self.hovered_tile == Some((x, y)) {
                    (0.8, 0.8, 0.8, 1.0)
                } else {
                    (0.3, 0.3, 0.3, 1.0)
                };
                renderer.draw_rect((r.0 + 1.0, r.1 + 1.0, r.2 - 1.0, r.3 - 1.0), color)?;
            }
        }

        for (position, r) in self.world.with_components::<(Position, ColoredRect)>() {
            let pos = position.0;
            let rect = (
                r.rect.0 as f32 + pos.0,
                r.rect.1 as f32 + pos.1,
                r.rect.2 as f32 + pos.0,
                r.rect.2 as f32 + pos.1,
            );
            renderer.draw_rect(rect, r.color)?;
        }

        for (position, c) in self.world.with_components::<(Position, ColoredCircle)>() {
            let center = Vec2::new(
                self.grid.cell_width() as f32 * 0.5,
                self.grid.cell_height() as f32 * 0.5,
            );
            renderer.draw_circle(position.0 + center, c.radius, 20, c.color)?;
        }

        Ok(())
    }
}
