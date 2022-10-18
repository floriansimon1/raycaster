const TARGET_ROTATIONS_PER_SECOND: u32 = 10;

use ggez::graphics::{Image, DrawParam, Drawable};

use crate::{raycaster, world::map_point_to_world_position, camera::Camera};
use crate::map::Map;

pub struct State {
    pub map:       Map,
    pub camera:    Camera,
    pub raycaster: raycaster::Raycaster,
}

impl State {
    pub fn new() -> Self {
        let map   = Map::make_demo_map();

        let spawn = map.find_first_spawn();

        State {
            map,

            raycaster: raycaster::Raycaster::new(),
            camera:    Camera::new(map_point_to_world_position(spawn), std::f32::consts::PI / 4.0),
        }
    }
}

impl ggez::event::EventHandler<ggez::GameError> for State {
    fn update(&mut self, context: &mut ggez::Context) -> ggez::GameResult {
        while ggez::timer::check_update_time(context, TARGET_ROTATIONS_PER_SECOND) {
            self.camera.rotate_clockwise(0.05 * 0.5);
        }

        std::thread::yield_now();

        Ok(())
    }

    fn draw(&mut self, context: &mut ggez::Context) -> ggez::GameResult {
        let image = Image::from_rgba8(
            context,
            raycaster::SCREEN_WIDTH,
            raycaster::SCREEN_HEIGHT,
            self.raycaster.update_framebuffer(&self.map, &self.camera)
        )?;

        image.draw(context, DrawParam::new())?;

        ggez::graphics::present(context)?;

        Ok(())
    }
}
