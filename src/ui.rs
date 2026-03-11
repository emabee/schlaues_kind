mod action;
mod assets;
mod modals;
pub mod the_app;
pub mod viz;

use crate::ui::assets::{IMG_BURGER, IMG_LOGO};
pub(crate) use action::Action;

// Window sizes
pub const WIN_WIDTH: f32 = 1400.;
pub const WIN_HEIGHT: f32 = 700.;
pub const WIN_MIN_HEIGHT: f32 = 200.;

pub const MEDIUM_MODAL_WIDTH: f32 = 800.;
pub const WIDE_MODAL_WIDTH: f32 = 1300.;
pub const SMALL_MODAL_WIDTH: f32 = 500.;
