use crate::{
    controller::Controller,
    data::Dictate,
    ui::{Action, MEDIUM_MODAL_WIDTH, modals::button},
};
use egui::{
    Color32, Context, FontFamily, FontId, Id, Modal, RichText, Sides, TextEdit, TextFormat,
    text::LayoutJob,
};

#[allow(clippy::too_many_arguments, clippy::too_many_lines)]
pub fn dictate(
    dictates: &[Dictate],
    current_dictate: &mut usize,
    current_line: &mut Option<usize>,
    edit_line: &mut String,
    edit_line_has_focus: &mut bool,
    check_is_on: &mut bool,
    controller: &mut Controller,
    ctx: &Context,
) {
    let dictate = &dictates[*current_dictate];
    Modal::new(Id::new("dictates_modal")).show(ctx, |ui| {
        ui.set_width(MEDIUM_MODAL_WIDTH);

        ui.horizontal(|ui| {
            ui.add_space(ui.available_width() / 2. - 100.);
            ui.label(
                RichText::new(t!("Kurze Diktate"))
                    .strong()
                    .font(FontId::new(20., FontFamily::Proportional)),
            );
        });

        ui.separator();

        ui.add_space(50.);

        ui.horizontal(|ui| {
            ui.label(
                RichText::new(&dictate.number)
                    .strong()
                    .font(FontId::new(30., FontFamily::Proportional))
                    .color(Color32::GRAY),
            );
            ui.label(
                RichText::new(dictate.title)
                    .strong()
                    .font(FontId::new(30., FontFamily::Proportional)),
            );
        });
        ui.add_space(10.);

        for (i, line) in &mut dictate.lines.iter().enumerate() {
            ui.horizontal(|ui| {
                if ui
                    .add(egui::RadioButton::new(*current_line == Some(i), ""))
                    .clicked()
                {
                    *current_line = Some(i);
                    edit_line.clear();
                }
                // ui.radio_value(current_line, Some(i), "");

                let color = if *edit_line_has_focus {
                    match current_line {
                        None => Color32::BLACK,
                        Some(n) => {
                            if *n == i {
                                // pub const LIGHT_BLUE: Self = Self::from_rgb(0xAD, 0xD8, 0xE6);
                                Color32::from_rgb(0xCD, 0xF8, 0xFF)
                            } else {
                                Color32::LIGHT_GRAY
                            }
                        }
                    }
                } else {
                    match current_line {
                        None => Color32::BLACK,
                        Some(n) => {
                            if *n == i {
                                Color32::BLUE
                            } else {
                                Color32::LIGHT_GRAY
                            }
                        }
                    }
                };

                ui.label(
                    RichText::new(*line)
                        .strong()
                        .font(FontId::new(20., FontFamily::Proportional))
                        .color(color),
                );
            });
        }

        ui.add_space(20.);

        // Edit line
        ui.horizontal(|ui| {
            *check_is_on = ui
                .button(RichText::new("☞").color(Color32::DARK_RED))
                .contains_pointer();
            ui.add_space(10.);
            match (*check_is_on, current_line) {
                (true, Some(idx)) => {
                    ui.add_space(4.);
                    ui.vertical(|ui| {
                        ui.add_space(1.5);
                        ui.label(diff_to_original(edit_line, dictate.lines[*idx]));
                        ui.add_space(0.1);
                    });
                }
                _ => {
                    *edit_line_has_focus = ui
                        .add(
                            TextEdit::singleline(edit_line)
                                .desired_width(600.)
                                .font(FontId::new(20., FontFamily::Proportional)),
                        )
                        .has_focus();
                }
            }
        });
        ui.add_space(50.);

        Sides::new().show(
            ui,
            |_ui| {},
            |ui| {
                if ui.button(button::next_item(&t!("_next_dictate"))).clicked() {
                    controller.set_action(Action::NextDictate);
                }
                if ui.button(button::cancel()).clicked() {
                    controller.set_action(Action::CloseModal);
                }
            },
        );
    });
}

fn diff_to_original(edit_line: &str, orig_line: &str) -> LayoutJob {
    let (good_part, starting_with_error) = split_at_diff(edit_line, orig_line);
    let mut job = LayoutJob::default();
    job.append(
        good_part,
        0.0,
        TextFormat {
            font_id: FontId::new(20.0, FontFamily::Proportional),
            color: Color32::from_rgba_premultiplied(0, 150, 0, 255), //medium dark green
            background: Color32::WHITE,
            ..Default::default()
        },
    );
    job.append(
        starting_with_error,
        0.0,
        TextFormat {
            font_id: FontId::new(20.0, FontFamily::Proportional),
            color: Color32::DARK_RED,
            background: Color32::LIGHT_RED,
            ..Default::default()
        },
    );

    if starting_with_error.is_empty() {
        let length_diff = orig_line.len() - good_part.len();
        if length_diff > 0 {
            job.append(
                &" ".repeat(length_diff),
                0.0,
                TextFormat {
                    font_id: FontId::new(20.0, FontFamily::Proportional),
                    color: Color32::RED,
                    background: Color32::LIGHT_RED,
                    ..Default::default()
                },
            );
        }
    }
    job
}

fn split_at_diff<'e>(edit_line: &'e str, orig_line: &str) -> (&'e str, &'e str) {
    let mut good_len = 0;
    for (e, o) in edit_line.chars().zip(orig_line.chars()) {
        if e != o {
            break;
        }
        good_len += e.len_utf8();
    }
    edit_line.split_at(good_len)
}

#[cfg(test)]
mod test {
    #[test]
    fn test_split_at_diff() {
        let (g, e) = super::split_at_diff("Hello World!", "Hello world!");
        assert_eq!(g, "Hello ");
        assert_eq!(e, "World!");
    }
}
