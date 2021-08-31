use anyhow::Result;
use bevy::{
    asset::{AssetLoader, LoadedAsset},
    prelude::*,
    reflect::TypeUuid
};
use std::{collections::HashMap, path::Path};

mod aseprite {
    use serde::Deserialize;

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Frame {
        // filename : String,
        pub duration: u32,
    }
    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct FrameTag {
        pub name: String,
        pub from: u32,
        pub to: u32,
        pub direction: String,
    }
    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Meta {
        pub app: String,
        pub version: String,
        pub image: String,
        pub format: String,
        pub scale: String,
        pub frame_tags: Vec<FrameTag>,
    }

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Desc {
        pub frames: Vec<Frame>,
        pub meta: Meta,
    }
}

#[derive(Debug, TypeUuid)]
#[uuid = "ab3a0ad8-6fbc-4528-a4a5-90e7bf3fa9e1"]
pub struct Spritesheet {
    pub image: String,
    pub ranges: HashMap<String, std::ops::Range<u32>>,
    pub durations: Vec<u32>,
}

impl Spritesheet {
    fn try_from_bytes(asset_path: &Path, bytes: Vec<u8>) -> Result<Spritesheet> {
        let desc: aseprite::Desc = serde_json::from_slice(&bytes[..]).unwrap();

        println!("desc: {:?}", desc);

        let ranges = desc
            .meta
            .frame_tags
            .iter()
            .map(|tag| (tag.name.clone(), tag.from..tag.to))
            .collect();

        let durations = desc.frames.iter().map(|f| f.duration).collect();

        let spritesheet = Spritesheet {
            image: "".into(),
            ranges,
            durations,
        };

        Ok(spritesheet)
    }
}

#[derive(Default)]
struct SpritesheetLoader {}

#[derive(Default)]
pub struct SpritesheetPlugin;

impl Plugin for SpritesheetPlugin {
    fn build(&self, app: &mut App) {
        app.add_asset::<Spritesheet>()
            .init_asset_loader::<SpritesheetLoader>();
    }
}

impl AssetLoader for SpritesheetLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, Result<(), anyhow::Error>> {
        Box::pin(async move {
            let path = load_context.path();
            let map = Spritesheet::try_from_bytes(path, bytes.into())?;
            load_context.set_default_asset(LoadedAsset::new(map));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        static EXTENSIONS: &[&str] = &["json"];
        EXTENSIONS
    }
}