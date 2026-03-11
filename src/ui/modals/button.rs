use egui::{Color32, RichText};

// pub(super) fn ok() -> RichText {
//     RichText::new(format!("✅ {}", t!("_ok"))).color(Color32::DARK_GREEN)
// }

pub(super) fn cancel() -> RichText {
    RichText::new(format!("❌ {}", t!("_cancel"))).color(Color32::DARK_RED)
}

pub(super) fn next_step(s: &str) -> RichText {
    RichText::new(format!("▶ {s}")).color(Color32::DARK_GRAY)
}

pub(super) fn next_item(s: &str) -> RichText {
    RichText::new(format!("▶▶ {s}")).color(Color32::DARK_GRAY)
}

pub(super) fn restart_item() -> RichText {
    RichText::new(format!("🔁 {}", t!("_repeat"))).color(Color32::DARK_GRAY)
}
