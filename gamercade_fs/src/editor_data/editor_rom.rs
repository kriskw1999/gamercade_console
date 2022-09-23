use std::{ffi::OsStr, path::PathBuf};

use gamercade_audio::SoundRom;
use gamercade_core::{FrameRate, GraphicsData, Resolution};
use serde::{Deserialize, Serialize};

use crate::GameAssetProvider;

use super::{EditorGraphicsData, EditorSoundData};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct EditorRom {
    pub resolution: Resolution,
    pub frame_rate: FrameRate,
    pub player_count: (usize, usize),
    pub graphics: EditorGraphicsData,
    pub sounds: EditorSoundData,
}

impl EditorRom {
    pub fn try_load(path: &PathBuf) -> Result<EditorRom, String> {
        let text = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
        serde_json::from_str::<EditorRom>(&text).map_err(|e| e.to_string())
    }

    pub fn try_save(&self, path: &PathBuf) -> Result<(), String> {
        let path = if path.extension() != Some(OsStr::new("gce")) {
            path.with_extension("gce")
        } else {
            path.clone()
        };

        std::fs::write(
            path,
            serde_json::to_string_pretty(self).expect("failed to serialize editor rom to json"),
        )
        .map_err(|e| e.to_string())
    }
}

impl Default for EditorRom {
    fn default() -> Self {
        Self {
            player_count: (1, 1),
            resolution: Resolution::default(),
            frame_rate: FrameRate::default(),
            graphics: EditorGraphicsData::default(),
            sounds: EditorSoundData::default(),
        }
    }
}

impl GameAssetProvider for EditorRom {
    fn resolution(&self) -> Resolution {
        self.resolution
    }

    fn frame_rate(&self) -> FrameRate {
        self.frame_rate
    }

    fn player_count(&self) -> (usize, usize) {
        self.player_count
    }

    fn graphics(&self) -> GraphicsData {
        (&self.graphics).into()
    }

    fn sounds(&self) -> SoundRom {
        (&self.sounds).into()
    }
}
