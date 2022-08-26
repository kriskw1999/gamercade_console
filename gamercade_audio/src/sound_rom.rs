use std::ops::Index;

use arrayvec::ArrayVec;
use serde::{Deserialize, Serialize};

use crate::{
    Chain, ChainId, EnvelopeDefinition, InstrumentDataDefinition, InstrumentId, Phrase, PhraseId,
    Song, SongId, WavetableDefinition, WavetableGenerator, WavetableWaveform,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoundRom {
    pub songs: Box<[Song]>,
    pub chains: Box<[Chain]>,
    pub phrases: Box<[Phrase]>,
    pub instruments: Box<[InstrumentDataDefinition]>,
    pub sfx: Box<[Sfx]>,
}

/// Represents a singular sound effect
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sfx {
    pub bpm: f32,
    pub chain: ChainId,
    // TODO: Should we include other data here, like
    // loop style? or should this be handled by game code?
}

impl Default for SoundRom {
    fn default() -> Self {
        let default_sine_wave = InstrumentDataDefinition::Wavetable(WavetableDefinition {
            data: WavetableGenerator {
                waveform: WavetableWaveform::Sine,
                size: 64,
            }
            .generate(),
            envelope: EnvelopeDefinition::interesting(),
        });

        let default_phrase = Phrase::c_scale(InstrumentId(0));

        let default_chain = Chain {
            entries: ArrayVec::from_iter(Some(Some(PhraseId(0)))),
        };

        let default_sfx = Sfx {
            bpm: 120.0,
            chain: ChainId(0),
        };

        Self {
            songs: vec![].into_boxed_slice(),
            chains: vec![default_chain].into_boxed_slice(),
            phrases: vec![default_phrase].into_boxed_slice(),
            instruments: vec![default_sine_wave].into_boxed_slice(),
            sfx: vec![default_sfx].into_boxed_slice(),
        }
    }
}

impl Index<SongId> for SoundRom {
    type Output = Song;

    fn index(&self, index: SongId) -> &Self::Output {
        &self.songs[index.0]
    }
}

impl Index<ChainId> for SoundRom {
    type Output = Chain;

    fn index(&self, index: ChainId) -> &Self::Output {
        &self.chains[index.0]
    }
}

impl Index<PhraseId> for SoundRom {
    type Output = Phrase;

    fn index(&self, index: PhraseId) -> &Self::Output {
        &self.phrases[index.0]
    }
}

impl Index<InstrumentId> for SoundRom {
    type Output = InstrumentDataDefinition;

    fn index(&self, index: InstrumentId) -> &Self::Output {
        &self.instruments[index.0]
    }
}