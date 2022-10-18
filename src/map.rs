use ggez::graphics::Color;

const DEMO_MAP_TILES_WIDTH:  MapCoordinate = 13;
const DEMO_MAP_TILES_HEIGHT: MapCoordinate = 18;

static DEMO_MAP_TILES: &'static [Tile] = &[
    T::Wall,  T::Wall,   T::Wall,   T::Wall,   T::Wall,   T::Wall,   T::Wall,   T::Wall,   T::Wall,   T::Wall,   T::Wall,   T::Wall,   T::Wall,
    T::Wall,  T::Empty,  T::Empty,  T::Empty,  T::Empty,  T::Empty,  T::Empty,  T::Empty,  T::Empty,  T::Empty,  T::Empty,  T::Empty,  T::Wall,
    T::Wall,  T::Empty,  T::Empty,  T::Empty,  T::Empty,  T::Empty,  T::Empty,  T::Empty,  T::Empty,  T::Empty,  T::Empty,  T::Empty,  T::Wall,
    T::Wall,  T::Empty,  T::Empty,  T::Empty,  T::Empty,  T::Empty,  T::Empty,  T::Empty,  T::Empty,  T::Empty,  T::Empty,  T::Empty,  T::Wall,
    T::Wall,  T::Empty,  T::Empty,  T::Empty,  T::Empty,  T::Empty,  T::Empty,  T::Empty,  T::Empty,  T::Empty,  T::Empty,  T::Empty,  T::Wall,
    T::Wall,  T::Empty,  T::Empty,  T::Empty,  T::Empty,  T::Empty,  T::Empty,  T::Empty,  T::Empty,  T::Empty,  T::Empty,  T::Empty,  T::Wall,
    T::Wall,  T::Empty,  T::Empty,  T::Empty,  T::Wall2,  T::Wall2,  T::Wall2,  T::Wall2,  T::Empty,  T::Empty,  T::Empty,  T::Empty,  T::Wall,
    T::Wall,  T::Empty,  T::Empty,  T::Empty,  T::Empty,  T::Empty,  T::Empty,  T::Wall2,  T::Empty,  T::Empty,  T::Empty,  T::Empty,  T::Wall,
    T::Wall,  T::Empty,  T::Empty,  T::Empty,  T::Empty,  T::Empty,  T::Empty,  T::Wall2,  T::Empty,  T::Empty,  T::Empty,  T::Empty,  T::Wall,
    T::Wall,  T::Empty,  T::Empty,  T::Empty,  T::Empty,  T::Spawn,  T::Empty,  T::Wall2,  T::Empty,  T::Empty,  T::Empty,  T::Empty,  T::Wall,
    T::Wall,  T::Empty,  T::Empty,  T::Empty,  T::Empty,  T::Empty,  T::Empty,  T::Empty,  T::Empty,  T::Empty,  T::Empty,  T::Empty,  T::Wall,
    T::Wall,  T::Empty,  T::Empty,  T::Empty,  T::Empty,  T::Empty,  T::Empty,  T::Empty,  T::Empty,  T::Empty,  T::Empty,  T::Empty,  T::Wall,
    T::Wall,  T::Empty,  T::Empty,  T::Empty,  T::Empty,  T::Empty,  T::Empty,  T::Empty,  T::Empty,  T::Empty,  T::Empty,  T::Empty,  T::Wall,
    T::Wall,  T::Empty,  T::Empty,  T::Empty,  T::Empty,  T::Empty,  T::Empty,  T::Empty,  T::Empty,  T::Empty,  T::Empty,  T::Empty,  T::Wall,
    T::Wall,  T::Empty,  T::Empty,  T::Empty,  T::Empty,  T::Empty,  T::Empty,  T::Empty,  T::Empty,  T::Empty,  T::Empty,  T::Empty,  T::Wall,
    T::Wall,  T::Empty,  T::Empty,  T::Empty,  T::Empty,  T::Empty,  T::Empty,  T::Empty,  T::Empty,  T::Empty,  T::Empty,  T::Empty,  T::Wall,
    T::Wall,  T::Empty,  T::Empty,  T::Empty,  T::Empty,  T::Empty,  T::Empty,  T::Empty,  T::Empty,  T::Empty,  T::Empty,  T::Empty,  T::Wall,
    T::Wall,  T::Wall,   T::Wall,   T::Wall,   T::Wall,   T::Wall,   T::Wall,   T::Wall,   T::Wall,   T::Wall,   T::Wall,   T::Wall,   T::Wall,
];

pub type MapCoordinate       = u32;
pub type SignedMapCoordinate = i64;

pub struct MapPosition {
    pub x: MapCoordinate,
    pub y: MapCoordinate,
}

pub struct SignedMapPosition {
    pub x: SignedMapCoordinate,
    pub y: SignedMapCoordinate,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Tile
{
    Wall,
    Empty,
    Spawn,
    Wall2,
}

type T = Tile;

impl Tile {
    pub fn color(&self) -> Option<Color> {
        match self {
            Tile::Wall2 => Some(Color::GREEN),
            Tile::Wall  => Some(Color::RED),
            Tile::Empty => None,
            Tile::Spawn => None,
        }
    }

    pub fn is_visible(&self) -> bool {
        match self {
            Tile::Wall2 => true,
            Tile::Wall  => true,
            Tile::Empty => false,
            Tile::Spawn => false,
        }
    }
}

pub struct Map {
    pub height: MapCoordinate,
    pub width:  MapCoordinate,
        tiles: &'static [Tile],
}

pub fn world_position_to_signed_map_position(position: glam::Vec2) -> SignedMapPosition {
    SignedMapPosition { x: position.x as SignedMapCoordinate, y: position.y as SignedMapCoordinate }
}

impl Map {
    pub fn make_demo_map() -> Map {
        Map { tiles: DEMO_MAP_TILES, width: DEMO_MAP_TILES_WIDTH, height: DEMO_MAP_TILES_HEIGHT }
    }

    pub fn find_first_spawn(&self) -> MapPosition {
        self
        .tiles
        .iter()
        .position(|tile| *tile == Tile::Spawn)
        .map(|index| {
            let index = index as MapCoordinate;

            let y = index / self.width;

            MapPosition { y, x: index % self.width }
        })
        .expect("No spawn found in map!")
    }

    pub(crate) fn get_tile(&self, x: MapCoordinate, y: MapCoordinate) -> Tile {
        self.tiles[(y * self.width + x) as usize]
    }
}
