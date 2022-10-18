#![allow(non_snake_case)]
#![allow(unused_parens)]

mod map;
mod world;
mod state;
mod camera;
mod raycaster;

fn main() {
    let mut config = ggez::conf::Conf::new();
    let     state  = state::State::new();

    config.window_mode.width  = raycaster::SCREEN_WIDTH as f32;
    config.window_mode.height = raycaster::SCREEN_HEIGHT as f32;
    config.window_setup       = config.window_setup.title("Raycaster");

    let (context, event_loop) = ggez::ContextBuilder::new("raycaster", "Florian")
    .default_conf(config)
    .build()
    .unwrap();

    ggez::event::run(context, event_loop, state);
}
