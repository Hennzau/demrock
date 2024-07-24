use std::f32::consts::FRAC_PI_2;

use macroquad::audio::{load_sound, Sound};
use macroquad::color::WHITE;
use macroquad::math::{vec2, Rect};
use macroquad::miniquad::FilterMode;
use macroquad::texture::{draw_texture_ex, load_texture, DrawTextureParams, Texture2D};
use rustc_hash::FxHashMap;
use serde::Deserialize;

use super::LineBorder;

pub const TILE_SIZE: f32 = 24.0;
pub const MAP_SIZE: (f32, f32) = (500.0, 250.0);

#[derive(Clone, Deserialize)]
pub struct Levels {
    pub levels: Vec<LevelConfig>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename(deserialize = "level"))]
pub struct LevelConfig {
    name: String,
    background_path: String,
    tiles_texture_path: String,
    music_path: String,
    starting_position: [usize; 2],
    tiles: Vec<Tile>,
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub struct Tile {
    pub position: [usize; 2],
    pub tile_type: TileType,
    pub rotation: Rotation,
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub enum TileType {
    StartingLine,
    Base1,
    Base2,
    Base3,
    Base4,
    Base5,
    Base6,
    HardTurnInterior,
    HardTurnExterior,
    SoftTurnInterior,
    SoftTurnInterior2,
    SoftTurnExterior,
    SoftTurnExterior2,
    StraightBorder,
    DiagBorder,
}

impl TileType {
    pub fn mapatlas_source(self) -> (f32, f32) {
        match self {
            TileType::StartingLine => (0.0, 0.0),
            TileType::HardTurnInterior => (1.0 * TILE_SIZE, 0.0),
            TileType::SoftTurnInterior => (2.0 * TILE_SIZE, 0.0),
            TileType::SoftTurnExterior => (3.0 * TILE_SIZE, 0.0),
            TileType::StraightBorder => (4.0 * TILE_SIZE, 0.0),
            TileType::Base1 => (0.0, 1.0 * TILE_SIZE),
            TileType::HardTurnExterior => (1.0 * TILE_SIZE, 1.0 * TILE_SIZE),
            TileType::SoftTurnInterior2 => (2.0 * TILE_SIZE, 1.0 * TILE_SIZE),
            TileType::SoftTurnExterior2 => (3.0 * TILE_SIZE, 1.0 * TILE_SIZE),
            TileType::DiagBorder => (4.0 * TILE_SIZE, 1.0 * TILE_SIZE),
            TileType::Base2 => (0.0, 2.0 * TILE_SIZE),
            TileType::Base3 => (1.0 * TILE_SIZE, 2.0 * TILE_SIZE),
            TileType::Base4 => (2.0 * TILE_SIZE, 2.0 * TILE_SIZE),
            TileType::Base5 => (3.0 * TILE_SIZE, 2.0 * TILE_SIZE),
            TileType::Base6 => (4.0 * TILE_SIZE, 2.0 * TILE_SIZE),
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize)]
pub enum Rotation {
    PiSur2 = 1,
    Pi = 2,
    PiFois3Sur2 = 3,
    PiFois2 = 0,
}

pub struct Level {
    name: String,
    pub background: Texture2D,
    pub tile_texture: Texture2D,
    music: Sound,
    pub starting_position: [usize; 2],
    tiles: Vec<Tile>,
    pub borders: FxHashMap<usize, LineBorder>,
}

impl Level {
    pub async fn load(conf: &LevelConfig) -> Self {
        let background: Texture2D = load_texture(&conf.background_path).await.expect("file bg");
        let music: Sound = load_sound(&conf.music_path).await.expect("file sound");
        let tile_texture: Texture2D = load_texture(&conf.tiles_texture_path)
            .await
            .expect("file tile");

        background.set_filter(FilterMode::Nearest);
        tile_texture.set_filter(FilterMode::Nearest);

        let borders: FxHashMap<usize, LineBorder> = FxHashMap::default();
        //borders.insert(tile_position_flatten(tile.position), LineBorder {start: conf.starting_position})

        Self {
            name: conf.name.clone(),
            background,
            tile_texture,
            music,
            starting_position: conf.starting_position,
            tiles: conf.tiles.clone(),
            borders,
        }
    }

    pub fn draw_background(&self) {
        draw_texture_ex(
            &self.background,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                ..Default::default()
            },
        );
    }

    pub fn draw_circuit(&self) {
        self.tiles.iter().for_each(|tile| {
            let (x, y) = tile.tile_type.mapatlas_source();
            draw_texture_ex(
                &self.tile_texture,
                TILE_SIZE * tile.position[0] as f32,
                TILE_SIZE * tile.position[1] as f32,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(TILE_SIZE, TILE_SIZE)),
                    source: Some(Rect::new(x, y, TILE_SIZE, TILE_SIZE)),
                    rotation: tile.rotation as usize as f32 * FRAC_PI_2,
                    ..Default::default()
                },
            )
        });
    }
}

#[inline]
pub fn tile_position_flatten(pos: [usize; 2]) -> usize {
    pos[0] * MAP_SIZE.0 as usize + pos[1] % MAP_SIZE.0 as usize
}
