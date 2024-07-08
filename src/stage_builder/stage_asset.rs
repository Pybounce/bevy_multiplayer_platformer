use bevy::utils::thiserror;
use bevy::{
    asset::{io::Reader, ron, AssetLoader, AsyncReadExt, LoadContext},
    prelude::*,
    reflect::TypePath,
    utils::BoxedFuture,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;


#[derive(Asset, TypePath, Debug, Deserialize, Serialize)]
pub struct Stage {
    pub tiles: Vec::<u32>,
    pub tiles_width: usize,
    pub tiles_height: usize,
    pub spawn_translation: Vec3
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

    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        _settings: &'a (),
        _load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;

            let custom_asset = ron::de::from_bytes::<Stage>(&bytes)?;
            Ok(custom_asset)
        })
    }

    fn extensions(&self) -> &[&str] {
        &["stage"]
    }

}