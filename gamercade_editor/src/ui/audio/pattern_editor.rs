use eframe::egui::Ui;

use crate::editor_data::EditorSoundData;

use super::AudioSyncHelper;

#[derive(Clone, Debug, Default)]
pub struct PatternEditor {}

impl PatternEditor {
    pub(crate) fn draw(
        &mut self,
        _ui: &mut Ui,
        _data: &mut EditorSoundData,
        _sync: &mut AudioSyncHelper,
    ) {
        // TODO: Write this
    }
}
