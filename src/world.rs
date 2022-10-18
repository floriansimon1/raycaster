use glam::Vec2;

use crate::map::MapPosition;

// Clockwise angle around the axis going up from the player's perspective.
pub type WorldAngle     = f32;
pub type WorldLength    = f32;
pub type WorldVector    = Vec2;
pub type WorldPosition  = Vec2;

pub type WorldDirection = WorldVector;

pub const  TILE_LENGTH:     f32         = 1.0;
pub static TOP_UNIT_VECTOR: WorldVector = WorldVector { x: 0.0, y: -1.0 };

pub fn map_point_to_world_position(position: MapPosition) -> WorldPosition {
    WorldPosition {
        x: position.x as f32 + TILE_LENGTH / 2.0,
        y: position.y as f32 + TILE_LENGTH / 2.0,
    }
}

pub fn rotate_clockwise(vector: WorldVector, radians: f32) -> WorldVector {
    let cos = radians.cos();
    let sin = radians.sin();

    // Rotation matrix
    // cos  -sin
    // sin   cos
    WorldVector {
        x: vector.x * cos - vector.y * sin,
        y: vector.x * sin + vector.y * cos,
    }
}

pub fn get_scaled_right_vector(x: f32) -> WorldVector {
    Vec2 { x, y: 0.0 }
}
