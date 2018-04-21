use embla::ecs::World;
use embla::input::{Input, MouseButton};
use embla::math::Vec2;
use failure::Error;

use grid::Grid;
use render_interface::RenderInterface;

struct Position(Vec2);
struct TilePosition(i32, i32);
struct ColoredRect {
    rect: (f32, f32, f32, f32),
    color: (f32, f32, f32, f32),
}

pub struct Game {
    grid: Grid,
    hovered_tile: Option<(i32, i32)>,
    screen_size: Vec2,
    world: World,
}

impl Game {
    pub fn new() -> Result<Game, Error> {
        Ok(Game {
            grid: Grid::new(10, 20, 20, 20),
            hovered_tile: None,
            screen_size: Vec2::new(0.0, 0.0),
            world: World::new(),
        })
    }

    pub fn update(&mut self, _dt: f32, input: &Input) -> Result<(), Error> {
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

        Ok(())
    }
}
