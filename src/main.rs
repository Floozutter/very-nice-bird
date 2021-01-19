#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    use ggez::{ContextBuilder, conf::{WindowSetup, WindowMode}};
    let builder = ContextBuilder::new("very-nice-bird", "Floozutter")
        .window_setup(
            WindowSetup::default()
                .title("living in a house under the wing of a very nice bird")
        )
        .window_mode(
            WindowMode::default()
                .dimensions(800.0, 600.0)
                .maximized(false)
                .resizable(false)
        );
    let (mut ctx, mut event_loop) = match builder.build() {
        Ok(build) => build,
        Err(error) => {
            eprintln!("error: couldn't build ggez context ({})!", error);
            std::process::exit(1);
        },
    };
    let mut state = State {};
    if let Err(error) = ggez::event::run(&mut ctx, &mut event_loop, &mut state) {
        eprintln!("error: main loop failed ({})!", error);
        std::process::exit(1);
    }
}

struct State {}
impl ggez::event::EventHandler for State {
    fn update(&mut self, _ctx: &mut ggez::Context) -> ggez::GameResult {
        Ok(())
    }
    fn draw(&mut self, _ctx: &mut ggez::Context) -> ggez::GameResult {
        Ok(())
    }
}
