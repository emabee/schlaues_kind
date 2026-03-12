use crate::{
    controller::Controller,
    data::WordList,
    ui::{Action, SMALL_MODAL_WIDTH, modals::button},
};
use egui::{Context, FontFamily, FontId, Id, Modal, RichText, Sides};

pub fn tricky_word(
    word_lists: &[WordList],
    word_list_index: &usize,
    word_idx: &usize,
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

        let mut new_word_list_index = *word_list_index;
        ui.horizontal(|ui| {
            egui::ComboBox::from_id_salt(t!("choose_list_type"))
                .selected_text(word_lists[new_word_list_index].description())
                .show_ui(ui, |ui| {
                    for (idx, word_list) in word_lists.iter().enumerate() {
                        ui.selectable_value(&mut new_word_list_index, idx, word_list.description());
                    }
                });
        });
        if *word_list_index != new_word_list_index {
            controller.set_action(Action::ChangeTrickyWordList(new_word_list_index));
        }

        ui.separator();

        ui.add_space(50.);

        ui.horizontal(|ui| {
            ui.add_space(50.);

            let word = &word_lists[*word_list_index].words[*word_idx];

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
