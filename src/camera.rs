use crate::{world::{WorldPosition, WorldAngle, WorldVector, rotate_clockwise, self, WorldDirection}};

/*
*       Cam segment
*    < -------------- >
*      \    ^      /
*       \   |    /
*        \  |  /
*         \ | /
*          V
*        Player
*/
pub struct Camera {
        field_of_view_ratio:     f32,
        rotation:                WorldAngle,
        cached_direction:        WorldVector,
        cached_camera_vector:    WorldVector,
    pub position:                WorldPosition,
}

impl Camera {
    pub fn new(position: WorldPosition, direction_angle_radians: WorldAngle) -> Camera {
        // Camera segment / camera direction vector's length (1).
        let field_of_view_ratio   = 0.20;

        let cached_direction      = Camera::get_direction(direction_angle_radians);
        let cached_camera_vector  = Camera::get_camera_vector(field_of_view_ratio, direction_angle_radians);

        Camera {
            position,
            cached_direction,
            field_of_view_ratio,
            cached_camera_vector,

            rotation: direction_angle_radians,
        }
    }

    pub fn rotate_clockwise(&mut self, direction_angle_radians: WorldAngle) {
        self.rotation             += direction_angle_radians;

        self.cached_direction      = Camera::get_direction(self.rotation);
        self.cached_camera_vector  = Camera::get_camera_vector(self.field_of_view_ratio, self.rotation);
    }

    pub fn get_ray_direction(&self, x: f32, width: f32) -> WorldDirection {
        let camera_vector_scale = (x / width - 0.5) * 2.0;

        (self.cached_direction + (camera_vector_scale * self.cached_camera_vector)).normalize()
    }

    fn get_camera_vector(ratio: f32, rotation_radians: f32) -> WorldVector {
        rotate_clockwise(world::get_scaled_right_vector(ratio), rotation_radians)
    }

    fn get_direction(rotation_radians: f32) -> WorldVector {
        rotate_clockwise(world::TOP_UNIT_VECTOR, rotation_radians)
    }
}
