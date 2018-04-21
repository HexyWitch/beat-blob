use embla::ecs::World;
use embla::input::Input;
use failure::Error;

use render_interface::RenderInterface;

pub struct Game {
    _world: World,
}

impl Game {
    pub fn new() -> Result<Game, Error> {
        Ok(Game {
            _world: World::new(),
        })
    }

    pub fn update(&mut self, _dt: f32, _input: &Input) -> Result<(), Error> {
        Ok(())
    }

    pub fn render(&mut self, _renderer: &mut RenderInterface) -> Result<(), Error> {
        Ok(())
    }
}
