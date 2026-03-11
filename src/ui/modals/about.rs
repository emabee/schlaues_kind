use crate::{
    GITHUB_LINK, PROG_TITLE, PROG_VERSION,
    controller::Controller,
    ui::{Action, IMG_LOGO, MEDIUM_MODAL_WIDTH},
};
use egui::{Color32, Context, FontFamily, FontId, Image, Modal, RichText, Sides};

pub fn show_about(controller: &mut Controller, ctx: &Context) {
    let modal_response = Modal::new("show_about".into()).show(ctx, |ui| {
        ui.set_width(MEDIUM_MODAL_WIDTH);
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.set_width(220.);
                ui.set_height(280.);
                ui.add_space(50.);
                ui.add(Image::new(IMG_LOGO));
            });

            ui.vertical(|ui| {
                ui.add_space(50.);
                ui.label(
                    RichText::new(PROG_TITLE).font(FontId::new(24., FontFamily::Proportional)),
                );
                ui.add_space(15.);
                ui.label(format!(
                    "{}\n\n{}: {}",
                    t!("_about_1"),
                    t!("Version"),
                    PROG_VERSION
                ));

                ui.add_space(10.);

                ui.label(
                    RichText::new(t!("_about_3")).font(FontId::new(11., FontFamily::Proportional)),
                );
                ui.hyperlink(GITHUB_LINK);
            });
        });

        ui.add_space(15.);
        ui.separator();
        ui.add_space(5.);

        Sides::new().show(
            ui,
            |_ui| {},
            |ui| {
                if ui
                    .button(
                        RichText::new("✅")
                            .color(Color32::DARK_GREEN)
                            .font(FontId::new(20., FontFamily::Proportional)),
                    )
                    .clicked()
                {
                    controller.set_action(Action::CloseModal);
                }
            },
        );
    });
    if modal_response.should_close() {
        controller.set_action(Action::CloseModal);
    }
}
