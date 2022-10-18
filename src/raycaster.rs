use ggez::graphics::Color;

use crate::{camera::Camera, map::{Map, MapCoordinate, Tile, world_position_to_signed_map_position}, world::{WorldDirection, WorldVector, WorldLength}};

pub const  SCREEN_HEIGHT:    u16 = 600;
pub const  SCREEN_WIDTH:     u16 = 800;
pub const  PIXEL_SIZE:       u16 = 4;

pub struct Raycaster {
    framebuffer: Vec<u8>,
    width:       f32,
}

pub struct RaycastHit {
    pub is_vertical: bool,
    pub tile:        Tile,
    pub distance:    WorldLength,
}

impl Raycaster {
    pub fn update_framebuffer(&mut self, map: &Map, camera: &Camera) -> &[u8] {
        for x in 0 .. SCREEN_WIDTH {
            self.render_scanline(map, camera, x);
        }

        &self.framebuffer
    }

    pub fn new() -> Raycaster {
        let framebuffer = vec![0; (SCREEN_WIDTH as usize) * (SCREEN_HEIGHT as usize) * (PIXEL_SIZE as usize)];

        Raycaster { framebuffer, width: SCREEN_WIDTH as f32 }
    }

    fn render_scanline(&mut self, map: &Map, camera: &Camera, x: u16) {
        let ray_direction = camera.get_ray_direction(x as f32, self.width);

        let hit           = self.cast_ray(map, camera, ray_direction);

        if hit.is_none() {
            return;
        }

        let hit           = hit.unwrap();

        let ceiling_color = Color::BLACK;
        let color         = hit.tile.color();
        let floor_color   = Color::new(0.5, 0.5, 0.5, 1.0);

        /*
        * Using triangle ratios, we determine that:
        *   unclipped_projected_height / distance_to_projection_plane (1 because of our camera setup)
        * = wall_height / distance_from_player
        *
        * We use 1 as our wall height.
        *
        * See https://www.permadi.com/tutorial/raycast/rayc9.html
        */
        let as_screen_height_ratio = 1.0 - num::clamp(1.0 / hit.distance, 0.0, 1.0);

        let empty_portion_size     = ((SCREEN_HEIGHT as f32) * as_screen_height_ratio / 2.0) as u16;

        let low_end                = empty_portion_size;
        let high_start             = SCREEN_HEIGHT - empty_portion_size;

        let mut color = color.unwrap();

        // This works because we operate in a grid.
        if !hit.is_vertical {
            color.r *= 0.5;
            color.g *= 0.5;
            color.b *= 0.5;
        }

        for y in 0 .. SCREEN_HEIGHT {
            self.set_pixel(x, y, &(
                if y < low_end {
                    floor_color
                } else if y > high_start {
                    ceiling_color
                } else {
                    color
                }
            ));
        }
    }

    fn cast_ray(&self, map: &Map, camera: &Camera, ray_direction: WorldDirection) -> Option<RaycastHit> {
        let mut within_bounds = true;
        let     x_pixel_sign  = if ray_direction.x < 0.0 { -1 } else { 1 };
        let     y_pixel_sign  = if ray_direction.y < 0.0 { -1 } else { 1 };
        let mut current_tile  = world_position_to_signed_map_position(camera.position);

        /*
        * When moving from 1 unit positively along the direction vector on a vector component (x or y),
        * we move the other by a factor `component B / component A`.
        *
        * Using Pythagora's theorem, we can find how much distance we need to travel to go from
        * one vector component (x or y) to the next. We need that because our algorithm moves
        * exactly one tile at a time (that allows to never go through any wall), and we need
        * to know which pixel should be visited next.
        *
        * We want to move the tile which is the "most" behind in terms of being ready to be
        * updated. If X is lagging behind, then we'll move horizontally, otherwise we'll move
        * vertically.
        *
        * In the case of the X component, when moving X 1 pixel to the right, we establish that:
        * - The distance travelled by X (distance_x) is exactly 1.
        * - The distance travelled by Y (distance_y) is exactly ray_direction.y / ray_direction.x.
        * - The distance travelled along the direction vector is sqrt(1² + (ray_direction.y / ray_direction.x)²).
        *
        * We can expand the distance along the direction vector as:
        * sqrt((ray_direction.x² + ray_direction.y²) / ray_direction.x²)
        *
        * Then take the denominator out of the square root:
        * sqrt(ray_direction.x² + ray.direction.y²) / ray_direction.x.
        *
        * The sqrt() part is exactly ray_direction's magnitude formula, which we know to be one
        * because direction vector are supposed to be normalized. That leaves us with:
        * 1 / ray_direction.x.
        *
        * We need to be careful: it is possible that the ray is perfectly horizontal or vertical,
        * which for the pupose of the DDA algorithm below, can be considered as a step that is,
        * never lagging behind, therefore always greater than the other one, therefore we can
        * set it to +∞.
        *
        * Also, we use absolute values because steps are for comparison only. We need to perform
        * comparisons on equal grounds.
        *
        * The same logic applies to Y, leaving us with:
        */
        let step = WorldVector {
            x: if ray_direction.x == 0.0 { f32::INFINITY } else { 1.0 / ray_direction.x.abs() },
            y: if ray_direction.y == 0.0 { f32::INFINITY } else { 1.0 / ray_direction.y.abs() },
        };

        /*
        * Our map coordinates are in the top-left corner of the tile, and our algorithm
        * needs to know which direction to advance first. Therefore, the initial
        * travelled distance might need to be adjusted so that the two coordinates
        * start fair in the "competition".
        */
        let mut steps_accumulator = WorldVector {
            x: if x_pixel_sign < 0 { 0.0 } else { step.x },
            y: if y_pixel_sign < 0 { 0.0 } else { step.y },
        };

        while within_bounds {
            let is_vertical_step = steps_accumulator.y < steps_accumulator.x;

            if is_vertical_step {
                current_tile.y += y_pixel_sign;
            } else {
                current_tile.x += x_pixel_sign;
            }

            let tile = map.get_tile(current_tile.x as MapCoordinate, current_tile.y as MapCoordinate);

            if tile.is_visible() {
                /*
                * We're in a convenient situation where walls are either vertical or horizontal, due to the map
                * setup. To determine if the wall is horizontal or vertical, we remember on which side the wall
                * was hit. Because of the infinity case, you can't trust both step values. Only one can
                * be trusted, we'll use the one that created the ray hit.
                *
                * We can't use euclidean distance to determine the distance to the wall because it give a weird
                * fishbowl effect. We're not interested in the distance between the eye of the player and the
                * obstacle, but, rather, by the distance from the obstacle to the camera plane located at
                * the player's position that is also perpandicular to the cast ray.
                *
                * Using triangle ratios (see https://lodev.org/cgtutor/images/raycastperpwalldist.png),
                * we can determine that
                * euclidean = (|ray_direction| * ray_y_distance) / ray_direction.y
                * in the vertical case.
                *
                * With triangle ratios, we also know that:
                * perpandicular / |player_direction| = euclidean / |ray_direction|.
                *
                * Which means that:
                *      perp = euclidean * |player_direction| / |ray_direction|. Using the value of euclidean:
                *           = ((|ray_direction| * ray_y_distance) / ray_direction.y) * |player_direction| / |ray_direction|
                *           = ray_y_distance / ray_direction.y * |player_direction|
                *
                * We can assume player_direction to have length 1 if we stop using ray_direction's values in calculations.
                * Instead, we'll try to factor out ray_direction by replacing it with known values.
                *
                * We then get perp = ray_y_distance / ray_direction.y.
                *
                * ray_y_distance = number_of_steps + corrective_step (because our grid cell size is 1, and rays are
                * cast from the outer borders of the cell.
                *
                * Steps accumulator measures the same thing, but instead of moving one by one, it moves step.y by step.y.
                *
                * We get steps_accumulator = (number_of_steps + corrective_step) * step.y
                *        so ray_y_distance = steps_accumulator.y / step.y
                *
                * Because step.y = 1 / ray_direction.y, perp = steps_accumulator.y / step.y / step.y
                *                                            = steps.accumulator.y
                */
                let euclidean_distance = (if is_vertical_step {
                    steps_accumulator.y
                } else {
                    steps_accumulator.x
                }).abs();

                return Some(RaycastHit { tile, is_vertical: is_vertical_step, distance: euclidean_distance });
            }

            // We increment steps after the iteration because we don't want to count steps into the wall.
            if is_vertical_step {
                steps_accumulator.y += step.y;
            } else {
                steps_accumulator.x += step.x;
            }

            within_bounds = (
                   current_tile.x >= 0
                && current_tile.y >= 0
                && (current_tile.x as MapCoordinate) < map.width
                && (current_tile.y as MapCoordinate) < map.height
            );
        }

        None
    }

    fn set_pixel(&mut self, x: u16, y: u16, color: &ggez::graphics::Color) {
        let big_x     = x as usize;
        let big_y     = y as usize;
        let big_width = self.width as usize;

        let start = (PIXEL_SIZE as usize) * (big_y * big_width + big_x);
        let rgba  = color.to_rgba();

        self.framebuffer[start + 0] = rgba.0;
        self.framebuffer[start + 1] = rgba.1;
        self.framebuffer[start + 2] = rgba.2;
        self.framebuffer[start + 3] = rgba.3;
    }
}
