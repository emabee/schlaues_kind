use crate::{
    controller::Controller,
    ui::{Action, SMALL_MODAL_WIDTH, modals::button, viz::WordCategory},
};
use egui::{Context, FontFamily, FontId, Id, Modal, RichText, Sides};

pub fn tricky_word(
    word_idx: &usize,
    tricky_short_words: &[String],
    tricky_words: &[String],
    tricky_long_words: &[String],
    selected_category: &mut WordCategory,
    controller: &mut Controller,
    ctx: &Context,
) {
    Modal::new(Id::new("tricky_words_modal")).show(ctx, |ui| {
        ui.set_width(SMALL_MODAL_WIDTH);

        ui.horizontal(|ui| {
            ui.add_space(ui.available_width() / 2. - 100.);
            ui.label(
                RichText::new(t!("Knifflige Wörter"))
                    .strong()
                    .font(FontId::new(20., FontFamily::Proportional)),
            );
        });

        ui.separator();

        let before = *selected_category;
        ui.horizontal(|ui| {
            egui::ComboBox::from_id_salt(t!("choose_list_type"))
                .selected_text(format!("{:?}", selected_category))
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        selected_category,
                        WordCategory::Short,
                        t!("Short_tricky_words"),
                    );
                    ui.selectable_value(
                        selected_category,
                        WordCategory::Medium,
                        t!("Medium_tricky_words"),
                    );
                    ui.selectable_value(
                        selected_category,
                        WordCategory::Long,
                        t!("Long_tricky_words"),
                    );
                });
        });
        if *selected_category != before {
            controller.set_action(Action::ChangeTrickyWordCategory);
            // we cannot render the UI in this frame, because category and index are not in sync
            // TODO: improve separation of UI and controller
            return;
        }

        ui.separator();

        ui.add_space(50.);

        ui.horizontal(|ui| {
            ui.add_space(50.);

            let word = match selected_category {
                WordCategory::Short => &tricky_short_words[*word_idx],
                WordCategory::Medium => &tricky_words[*word_idx],
                WordCategory::Long => &tricky_long_words[*word_idx],
            };

            ui.label(
                RichText::new(word)
                    .strong()
                    .font(FontId::new(30., FontFamily::Proportional)),
            );
        });

        ui.add_space(50.);

        Sides::new().show(
            ui,
            |_ui| {},
            |ui| {
                if ui
                    .button(button::next_item(&t!("_next_tricky_word")))
                    .clicked()
                {
                    controller.set_action(Action::NextTrickyWord);
                }
                if ui.button(button::cancel()).clicked() {
                    controller.set_action(Action::CloseModal);
                }
            },
        );
    });
}
