use std::sync::Arc;

use arrayvec::ArrayVec;
use rodio::{OutputStream, Source};

use gamercade_audio::{
    initialize_luts, Chain, ChainId, ChainPlayback, EnvelopeDefinition, InstrumentDefinition,
    InstrumentId, InstrumentInstance, PatchDefinition, Phrase, PhraseId, Song, SongId, SoundEngine,
    SoundRom, TrackerFlow, WavetableDefinition, WavetableGenerator, WavetableWaveform,
    PHRASE_STEPS_PER_BEAT,
};

fn main() {
    // Initialization
    initialize_luts();
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    let engine = test_engine();

    let mut chain = ChainPlayback::new(ChainId(0), &engine);
    let instrument_instance = InstrumentInstance::from(&engine[InstrumentId(0)]);

    let engine = Arc::new(engine);

    let entries_per_second = 60.0 / engine[SongId(0)].bpm / PHRASE_STEPS_PER_BEAT as f32;

    let instrument_instance = instrument_instance.periodic_access(
        std::time::Duration::from_secs_f32(entries_per_second),
        move |instance| {
            if TrackerFlow::Finished == chain.update_tracker(&engine, instance) {
                chain = ChainPlayback::new(ChainId(0), &engine);
            }
            // phrase.adjust_instrument_instance(&engine, instance);
            // phrase.next_step();
        },
    );

    stream_handle.play_raw(instrument_instance).unwrap();
    std::thread::sleep(std::time::Duration::from_secs_f32(25.0));
}

fn test_engine() -> SoundEngine {
    // Initialize our sound sources
    let instruments = vec![
        InstrumentDefinition::FMSynth(PatchDefinition::default()),
        InstrumentDefinition::Wavetable(WavetableDefinition {
            data: WavetableGenerator {
                waveform: WavetableWaveform::Sine,
                size: 64,
            }
            .generate(),
            envelope: EnvelopeDefinition::interesting(),
            sample_rate: 44_100, //44.1 khz
        }),
    ];

    let mut chains = ArrayVec::new();
    chains.push(Some(PhraseId(0)));
    chains.push(Some(PhraseId(1)));

    let songs = vec![Song {
        bpm: 120.0,
        tracks: vec![[Some(ChainId(0)), None, None, None, None, None, None, None]]
            .into_boxed_slice(),
    }]
    .into_boxed_slice();

    let rom = SoundRom {
        songs,
        chains: vec![Chain { entries: chains }].into_boxed_slice(),
        phrases: vec![Phrase::c_scale(), Phrase::c_scale_reverse()].into_boxed_slice(),
        instruments: instruments.into_boxed_slice(),
    };

    SoundEngine::initialize(rom)
}
