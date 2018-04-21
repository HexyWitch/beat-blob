use embla::ecs::World;
use embla::input::{Input, MouseButton};
use embla::math::Vec2;
use failure::Error;

use grid::Grid;
use render_interface::RenderInterface;
use systems;

use components::{BlobGoal, BlobSpawn, ColoredCircle, ColoredRect, FillMode, Pad, PadTeam,
                 Position, TilePosition};

const BEAT_TIME: f32 = 0.25;

pub struct Game {
    grid: Grid,
    hovered_tile: Option<(i32, i32)>,
    screen_size: Vec2,
    world: World,
    beat_timer: f32,
}

impl Game {
    pub fn new() -> Result<Game, Error> {
        let mut game = Game {
            grid: Grid::new(9, 10, 40, 40),
            hovered_tile: None,
            screen_size: Vec2::new(0.0, 0.0),
            world: World::new(),
            beat_timer: 0.0,
        };

        game.init()?;

        Ok(game)
    }

    fn init(&mut self) -> Result<(), Error> {
        self.insert_pad(1, 1, PadTeam::Blue)?;
        self.insert_pad(3, 1, PadTeam::Red)?;
        self.insert_pad(5, 1, PadTeam::Green)?;
        self.insert_pad(7, 1, PadTeam::Yellow)?;

        self.insert_spawn(1, 9, PadTeam::Blue, 2)?;
        self.insert_spawn(3, 9, PadTeam::Red, 4)?;
        self.insert_spawn(5, 9, PadTeam::Green, 6)?;
        self.insert_spawn(7, 9, PadTeam::Yellow, 8)?;

        self.insert_goal(1, 0, PadTeam::Blue)?;
        self.insert_goal(3, 0, PadTeam::Red)?;
        self.insert_goal(5, 0, PadTeam::Green)?;
        self.insert_goal(7, 0, PadTeam::Yellow)?;

        Ok(())
    }

    pub fn update(&mut self, dt: f32, input: &Input) -> Result<(), Error> {
        let mut mouse_position = input.mouse_position();
        mouse_position.1 = self.screen_size.1 as f32 - mouse_position.1;
        self.hovered_tile = self.grid.tile_at(mouse_position);

        if let Some(tile) = self.hovered_tile {
            if input.mouse_button_is_down(&MouseButton::Left) {
                self.insert_wall(tile)?;
            }
        }

        self.beat_timer += dt;
        // On beat
        while self.beat_timer > BEAT_TIME {
            self.beat_timer -= BEAT_TIME;

            systems::move_blobs(&mut self.world)?;
            systems::spawn_blobs(&self.grid, &mut self.world)?;
        }
        systems::grid_positioning(&self.grid, &mut self.world)?;

        // do all tweening after grid positioning
        let beat_ratio = self.beat_timer / BEAT_TIME;
        systems::tween_blobs(beat_ratio, &self.grid, &mut self.world)?;

        systems::pad_update(dt, input, &self.grid, &mut self.world)?;

        systems::trigger_blobs(&mut self.world)?;

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

        systems::render_primitives(&mut self.world, renderer)?;

        Ok(())
    }

    fn insert_wall(&mut self, (x, y): (i32, i32)) -> Result<(), Error> {
        let size = (self.grid.cell_width(), self.grid.cell_height());
        let rect = (1.0, 1.0, size.0 as f32 - 1.0, size.1 as f32 - 1.0);
        self.world
            .add_entity()
            .insert(Position(Vec2::zero()))
            .insert(TilePosition(x, y))
            .insert(ColoredRect {
                rect,
                color: (1.0, 1.0, 1.0, 1.0),
            });

        self.grid.set_occupied((x, y), true);

        Ok(())
    }

    fn insert_spawn(&mut self, x: i32, y: i32, team: PadTeam, interval: i32) -> Result<(), Error> {
        self.world
            .add_entity()
            .insert(Position(Vec2::zero()))
            .insert(TilePosition(x, y))
            .insert(team)
            .insert(ColoredCircle {
                radius: self.grid.cell_width() as f32 * 0.35,
                color: team.color(),
                fill: FillMode::Outline(2.0),
            })
            .insert(BlobSpawn {
                interval: interval,
                timer: interval,
            });

        Ok(())
    }

    fn insert_goal(&mut self, x: i32, y: i32, team: PadTeam) -> Result<(), Error> {
        self.world
            .add_entity()
            .insert(Position(Vec2::zero()))
            .insert(TilePosition(x, y))
            .insert(team)
            .insert(ColoredCircle {
                radius: self.grid.cell_width() as f32 * 0.45,
                color: team.color(),
                fill: FillMode::Outline(2.0),
            })
            .insert(BlobGoal);

        Ok(())
    }

    fn insert_pad(&mut self, x: i32, y: i32, team: PadTeam) -> Result<(), Error> {
        self.world
            .add_entity()
            .insert(Position(Vec2::zero()))
            .insert(TilePosition(x, y))
            .insert(ColoredCircle {
                radius: self.grid.cell_width() as f32 * 0.35,
                color: team.color(),
                fill: FillMode::Outline(6.0),
            })
            .insert(team)
            .insert(Pad { pulse_timer: 0.0 });

        Ok(())
    }
}
