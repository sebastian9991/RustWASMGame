use quicksilver::{
    geom::Vector,
    graphics::{Color, Image, VectorFont},
    run, Graphics, Input, Result, Settings, Window,
};
struct Game {
    title: Image,
    font_info: Image,
}

impl Game {
    // Load the assets, initialise the Game
    async fn new(window: Window, mut gfx: Graphics, mut input: Input) -> Result<Self> {
        gfx.clear(Color::BLACK);
        let ttf = VectorFont::load("mononoki-Regular.ttf").await?;
        let title = ttf.to_renderer(&gfx, 72.0);
        Ok(Self { title, font_info })
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
        title: "The rouge game",
        ..Default::default()
    };
    run(settings, app)
}

async fn app(window: Window, mut gfx: Graphics, mut input: Input) -> Result<()> {
    gfx.present(&window);
    loop {
        while let Some(_) = input.next_event().await {}
    }
}
