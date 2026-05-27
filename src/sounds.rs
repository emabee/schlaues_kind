use rodio::{Decoder, MixerDeviceSink};
use std::io::Cursor;

use crate::assets::{
    SOUND_BELL_CHORD, SOUND_BELL_DING, SOUND_CHRISTMAS_BELL, SOUND_HAND_BELL, SOUND_NICE,
    SOUND_SHIP_BELL,
};

#[derive(Clone, Copy)]
pub enum Sound {
    BellDing,
    BellChord,
    ChristmasBell,
    HandBell,
    ShipBell,
    Nice,
}
impl Sound {
    pub fn play(self, sink: &MixerDeviceSink) {
        sink.mixer().add(
            Decoder::try_from(Cursor::new(match self {
                Self::BellDing => &SOUND_BELL_DING,
                Self::BellChord => &SOUND_BELL_CHORD,
                Self::ChristmasBell => &SOUND_CHRISTMAS_BELL,
                Self::HandBell => &SOUND_HAND_BELL,
                Self::ShipBell => &SOUND_SHIP_BELL,
                Self::Nice => &SOUND_NICE,
            }))
            .unwrap(),
        );
    }
}
