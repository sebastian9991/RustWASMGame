use quicksilver::{geom::Vector, run, Graphics, Input, Result, Settings, Window};
struct Game;

impl State for Game {
    // Load the assets, initialise the Game
    fn new() -> Result<Self> {
        Ok(Self)
    }

    //Process keybourd, mouse input to update the game state
    fn update(&mut self, window: &mut Window) -> Result<()> {
        Ok(())
    }

    //Draw stuff on screen
    fn draw(&mut self, window: &mut Window) -> Result<()> {
        Ok(())
    }
}

fn main() {
    let settings = Settings {
        ..Default::default()
    };
    run(settings, app)
}

async fn app(window: Window, mut gfx: Graphics, mut input: Input) -> Result<()> {}
