use failure::Error;

use embla;
use embla::input::Input;

use game::Game;
use renderer::GameRenderer;

pub struct Application {
    renderer: GameRenderer<embla::Renderer>,
    client: Game,
}

impl Application {
    pub fn new() -> Result<Self, Error> {
        Ok(Application {
            renderer: GameRenderer::<embla::Renderer>::new()?,
            client: Game::new()?,
        })
    }

    pub fn update(&mut self, dt: f32, input: &Input) -> Result<(), Error> {
        self.client.update(dt, input)?;

        self.client.render(&mut self.renderer)?;

        self.renderer.do_render().unwrap();

        Ok(())
    }
}
