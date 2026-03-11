use crate::ui::{WIN_HEIGHT, WIN_MIN_HEIGHT, WIN_WIDTH, the_app::TheApp};
use anyhow::{Result, anyhow};
use eframe::{NativeOptions, run_native};
use egui::{IconData, ViewportBuilder};
use egui_extras::install_image_loaders;
use image::{DynamicImage, ImageError, ImageFormat, ImageReader};
use rust_i18n::set_locale;
use std::{
    io::{BufReader, Cursor},
    process::ExitCode,
};
#[macro_use]
extern crate rust_i18n;

i18n!("locales", fallback = "en");

mod controller;
mod data;
mod ui;

pub const PROG_NAME: &str = env!("CARGO_PKG_NAME");
pub const PROG_TITLE: &str = "Schlaues Kind!";
pub const PROG_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const GITHUB_LINK: &str = "https://github.com/emabee/rust-schlaues_kind";

pub type Language = (&'static str, &'static str);
pub const SUPPORTED_LANGUAGES: [Language; 2] = [("en", "English"), ("de", "Deutsch")];
pub const DEFAULT_LANGUAGE: &Language = &SUPPORTED_LANGUAGES[0];

fn main() -> ExitCode {
    set_locale(SUPPORTED_LANGUAGES[1].0); // todo: make this configurable
    i18n!("locales", fallback = "en");

    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            println!("Error occured: {e:?}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<()> {
    run_native(
        PROG_TITLE,
        NativeOptions {
            // viewport = native OS window
            viewport: ViewportBuilder::default()
                .with_inner_size([WIN_WIDTH, WIN_HEIGHT])
                .with_min_inner_size([WIN_WIDTH, WIN_MIN_HEIGHT])
                .with_app_id(PROG_TITLE)
                .with_icon(load_icon()),
            ..Default::default()
        },
        Box::new(|cc| {
            install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(
                // hand instance of App over to eframe::run_native,
                // which will then call its method `update()` in an endless loop
                TheApp::new()?,
            ))
        }),
    )
    .map_err(|e| anyhow!("Couldn't start GUI, caused by {e:?}"))
    .unwrap();

    Ok(())
}

fn load_icon() -> IconData {
    if let Ok(image) = read_logo() {
        IconData {
            rgba: image.to_rgba8().as_flat_samples().as_slice().to_vec(),
            width: image.width(),
            height: image.height(),
        }
    } else {
        IconData::default()
    }
}
fn read_logo() -> Result<DynamicImage, ImageError> {
    let bytes = include_bytes!("ui/assets/logo.png");
    ImageReader::with_format(BufReader::new(Cursor::new(bytes)), ImageFormat::Png).decode()
}
