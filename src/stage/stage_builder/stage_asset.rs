

use bevy::{
    asset::{io::Reader, ron, AssetLoader, AsyncReadExt, LoadContext},
    prelude::*,
    reflect::TypePath,
};
use serde::{ Deserialize, Serialize};
use thiserror::Error;

#[derive(Asset, TypePath, Debug, Deserialize, Serialize)]
pub struct Stage {
    pub id: usize,
    pub ground_tiles: Vec<GroundTile>,
    pub spikes: Vec<Spike>,
    pub half_saws: Vec<HalfSaw>,
    pub springs: Vec<Spring>,
    pub lock_blocks: Vec<LockBlock>,
    pub keys: Vec<Key>,
    pub checkpoints: Vec<Checkpoint>,
    pub grid_width: usize,
    pub grid_height: usize,
    pub spawn_grid_pos: Vec2,
    pub goal_grid_pos: Vec2
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GroundTile {
    pub grid_pos: Vec2,
    pub tilemap_index: usize
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Spike {
    pub grid_pos: Vec2,
    pub rotation: f32
}

#[derive(Debug, Deserialize, Serialize)]
pub struct HalfSaw {
    pub grid_pos: Vec2,
    pub rotation: f32
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Checkpoint {
    pub grid_pos: Vec2
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Spring {
    pub grid_pos: Vec2,
    pub rotation: f32
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LockBlock {
    pub grid_pos: Vec2,
    pub trigger_id: usize
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Key {
    pub grid_pos: Vec2,
    pub trigger_id: usize
}

#[derive(Default)]
pub struct StageLoader;

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum StageLoaderError {
    /// An [IO](std::io) Error
    #[error("Could not load asset: {0}")]
    Io(#[from] std::io::Error),
    /// A [RON](ron) Error
    #[error("Could not parse RON: {0}")]
    RonSpannedError(#[from] ron::error::SpannedError),

}


impl AssetLoader for StageLoader {
    type Asset = Stage;
    type Settings = ();
    type Error = StageLoaderError;

    async fn load<'a>(
        &'a self,
        reader: &'a mut Reader<'_>,
        _settings: &'a (),
        _load_context: &'a mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;

            let custom_asset = ron::de::from_bytes::<Stage>(&bytes)?;
            Ok(custom_asset)
    }

    fn extensions(&self) -> &[&str] {
        &["stage"]
    }

}