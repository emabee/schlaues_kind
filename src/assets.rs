use egui::{ImageSource, include_image};
use include_optional::include_str_optional;

// Lists
pub const IRREGULAR_VERBS: &str = include_str!("./assets/lists/Alle_irregulären_Verben.txt");

pub const MEDIUM_TRICKY_WORDS: &str = include_str!("./assets/lists/Knifflige_Wörter.txt");
pub const LONG_TRICKY_WORDS: &str = include_str!("./assets/lists/Lange_knifflige_Wörter.txt");
pub const SHORT_TRICKY_WORDS: &str = include_str!("./assets/lists/Kurze_knifflige_Wörter.txt");

pub const DICTATES: [Option<&str>; 2] = [
    include_str_optional!("./assets/lists/Diktate 2.txt"),
    Some(include_str!("./assets/lists/Diktate 3 4.txt")),
];
pub const DICTATE_2: usize = 0;
pub const DICTATE_34: usize = 1;

// Sounds
pub const SOUND_BELL_DING: &[u8] =
    include_bytes!("./assets/sounds/floraphonic-copper-bell-ding-22-172687.mp3");
pub const SOUND_BELL_CHORD: &[u8] =
    include_bytes!("./assets/sounds/freesound_community-bell-chord1-83260.mp3");
pub const SOUND_CHRISTMAS_BELL: &[u8] =
    include_bytes!("./assets/sounds/freesound_community-christmas-bell-3-48059.mp3");
pub const SOUND_HAND_BELL: &[u8] =
    include_bytes!("./assets/sounds/freesound_community-handbell-81953.mp3");
pub const SOUND_SHIP_BELL: &[u8] =
    include_bytes!("./assets/sounds/freesound_community-ship-bell-two-chimes-102730.mp3");
pub const SOUND_NICE: &[u8] = include_bytes!("./assets/sounds/jacqtydus-nice-392261.mp3");

// Images
macro_rules! img {
    ($img_name:ident, $img_file:literal) => {
        pub(super) const $img_name: ImageSource = include_image!($img_file);
    };
}
img!(BURGER_IMG, "./assets/ui/burger.png");
img!(LOGO_IMG, "./assets/ui/logo.png");
pub const LOGO_BYTES: &[u8; 39320] = include_bytes!("./assets/ui/logo.png");
