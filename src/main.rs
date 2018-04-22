extern crate embla;
extern crate failure;

mod application;
mod components;
mod game;
mod grid;
mod render_interface;
mod renderer;
mod systems;

use application::Application;

pub fn main() {
    embla::run(|| {
        let mut application = Application::new().unwrap();
        move |dt, input| {
            application.update(dt, input)?;

            Ok(())
        }
    });
}
