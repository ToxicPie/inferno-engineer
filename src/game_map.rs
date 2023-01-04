use bevy::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;
use serde::Deserialize;
use std::collections::HashMap;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RonAssetPlugin::<MapFile>::new(&["map.ron"]))
            .init_resource::<MapTileset>()
            .init_resource::<MapFileHandle>()
            .init_resource::<Map>()
            .add_startup_system(load_files)
            .add_system(prepare_map);
    }
}

fn load_files(
    asset_server: Res<AssetServer>,
    mut map_handle: ResMut<MapFileHandle>,
    mut tileset: ResMut<MapTileset>,
) {
    map_handle.0 = asset_server.load("mapfile.map.ron");
    for tile_type in 0..16 {
        let filename = format!("bg/tile{}.png", tile_type);
        tileset.0.insert(tile_type, asset_server.load(filename));
    }
}

fn prepare_map(
    mut map: ResMut<Map>,
    map_handle: ResMut<MapFileHandle>,
    map_file: Res<Assets<MapFile>>,
) {
    if map.loaded {
        return;
    }
    let Some(map_file) = map_file.get(&map_handle.0) else { return; };

    let map_rows: Vec<Vec<char>> = map_file
        .tiles
        .iter()
        .rev()
        .map(|s| s.chars().collect())
        .collect();

    map.width = map_file.width;
    map.height = map_file.height;
    map.start_pos = map_file.start_pos;
    for x in 0..map.width {
        let mut row = vec![];
        for y in 0..map.height {
            let tile = if map_rows[y][x] != '.' {
                Tile::empty()
            } else {
                let right_neighbor = x + 1 < map.width && map_rows[y][x + 1] == '.';
                let up_neighbor = y + 1 < map.height && map_rows[y + 1][x] == '.';
                let left_neighbor = x > 0 && map_rows[y][x - 1] == '.';
                let down_neighbor = y > 0 && map_rows[y - 1][x] == '.';
                Tile::from_neighbors(right_neighbor, up_neighbor, left_neighbor, down_neighbor)
            };
            row.push(tile);
        }
        map.tiles.push(row);
    }

    map.loaded = true;
}

#[derive(Resource)]
pub struct Tile {
    pub tile_type: usize,
    neighbors: (bool, bool, bool, bool),
}

#[derive(Resource, Default)]
pub struct Map {
    pub width: usize,
    pub height: usize,
    pub start_pos: (usize, usize),
    pub tiles: Vec<Vec<Tile>>,
    pub loaded: bool,
}

#[derive(Deserialize, bevy::reflect::TypeUuid)]
#[uuid = "0ec4d9f0-7c50-4630-9548-d2cf45eaf106"]
struct MapFile {
    width: usize,
    height: usize,
    start_pos: (usize, usize),
    tiles: Vec<String>,
}

#[derive(Resource, Default)]
struct MapFileHandle(Handle<MapFile>);

#[derive(Resource, Default)]
pub struct MapTileset(pub HashMap<usize, Handle<Image>>);

impl Tile {
    pub const WIDTH: f32 = 320.0;
    pub const HEIGHT: f32 = 240.0;
    pub const Z_LAYER: f32 = 0.0;
    const ROAD_WIDTH: f32 = 50.0;
    const ROAD_HEIGHT: f32 = 50.0;
    fn empty() -> Tile {
        Tile {
            tile_type: 0,
            neighbors: (false, false, false, false),
        }
    }
    fn from_neighbors(right: bool, up: bool, left: bool, down: bool) -> Tile {
        let tile_type = if right { 1 << 0 } else { 0 }
            | if up { 1 << 1 } else { 0 }
            | if left { 1 << 2 } else { 0 }
            | if down { 1 << 3 } else { 0 };
        Tile {
            tile_type,
            neighbors: (left, right, up, down),
        }
    }
    // returns if the given offset position is on some road
    pub fn is_valid(&self, offset_x: f32, offset_y: f32) -> bool {
        if self.tile_type == 0 {
            return false;
        }
        let is_left = offset_x < (Tile::WIDTH - Tile::ROAD_WIDTH) / 2.0;
        let is_right = offset_x > (Tile::WIDTH + Tile::ROAD_WIDTH) / 2.0;
        let is_mid_h = !is_left && !is_right;
        let is_up = offset_y > (Tile::HEIGHT + Tile::ROAD_HEIGHT) / 2.0;
        let is_down = offset_y < (Tile::HEIGHT - Tile::ROAD_HEIGHT) / 2.0;
        let is_mid_v = !is_up && !is_down;
        (is_mid_h && is_mid_v)
            || self.neighbors.0 && is_left && is_mid_v
            || self.neighbors.1 && is_right && is_mid_v
            || self.neighbors.2 && is_up && is_mid_h
            || self.neighbors.3 && is_down && is_mid_h
    }
}
