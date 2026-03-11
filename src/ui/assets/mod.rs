use egui::{ImageSource, include_image};

// Provide unique access to the images in this folder
macro_rules! img {
    ($img_name:ident, $img_file:literal) => {
        pub(super) const $img_name: ImageSource = include_image!($img_file);
    };
}

img!(IMG_BURGER, "burger.png");
img!(IMG_LOGO, "logo.png");
