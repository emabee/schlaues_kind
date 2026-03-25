use rodio::{Decoder, MixerDeviceSink};
use std::io::Cursor;

const BELL_DING: &[u8] =
    include_bytes!("./../assets/sounds/floraphonic-copper-bell-ding-22-172687.mp3");
const BELL_CHORD: &[u8] =
    include_bytes!("./../assets/sounds/freesound_community-bell-chord1-83260.mp3");
const CHRISTMAS_BELL: &[u8] =
    include_bytes!("./../assets/sounds/freesound_community-christmas-bell-3-48059.mp3");
const HAND_BELL: &[u8] =
    include_bytes!("./../assets/sounds/freesound_community-handbell-81953.mp3");
const SHIP_BELL: &[u8] =
    include_bytes!("./../assets/sounds/freesound_community-ship-bell-two-chimes-102730.mp3");
const NICE: &[u8] = include_bytes!("./../assets/sounds/jacqtydus-nice-392261.mp3");

pub fn play(sink: &MixerDeviceSink, sound: Sound) {
    match sound {
        Sound::BellDing => sink
            .mixer()
            .add(Decoder::try_from(Cursor::new(&BELL_DING)).unwrap()),
        Sound::BellChord => sink
            .mixer()
            .add(Decoder::try_from(Cursor::new(&BELL_CHORD)).unwrap()),
        Sound::ChristmasBell => sink
            .mixer()
            .add(Decoder::try_from(Cursor::new(&CHRISTMAS_BELL)).unwrap()),
        Sound::HandBell => sink
            .mixer()
            .add(Decoder::try_from(Cursor::new(&HAND_BELL)).unwrap()),
        Sound::ShipBell => sink
            .mixer()
            .add(Decoder::try_from(Cursor::new(&SHIP_BELL)).unwrap()),
        Sound::Nice => sink
            .mixer()
            .add(Decoder::try_from(Cursor::new(&NICE)).unwrap()),
    }
}

#[derive(Clone, Copy)]
pub enum Sound {
    BellDing,
    BellChord,
    ChristmasBell,
    HandBell,
    ShipBell,
    Nice,
}
