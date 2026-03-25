use crate::{
    controller::Controller,
    data::Verb,
    ui::{Action, WIDE_MODAL_WIDTH, modals::button},
};
use egui::{Align, Color32, Context, FontFamily, FontId, Id, Layout, Modal, RichText, Sides, Ui};
use egui_extras::{Column, TableBody, TableBuilder};

const T_PRAESENS: &str = "Präsens";
const T_PRAETERITUM: &str = "Präteritum";
const T_PERFECT: &str = "Perfekt";

pub fn verb_declination(
    verb_idx: usize,
    viz_level: isize,
    verbs: &[Verb],
    controller: &mut Controller,
    ctx: &Context,
) {
    let verb = &verbs[verb_idx];

    Modal::new(Id::new("verbs_modal")).show(ctx, |ui| {
        ui.set_width(WIDE_MODAL_WIDTH);

        ui.horizontal(|ui| {
            ui.add_space(ui.available_width() / 2. - 100.);
            ui.label(
                RichText::new(verb.infinitiv)
                    .strong()
                    .font(FontId::new(20., FontFamily::Proportional)),
            );
        });
        ui.separator();

        let mut viz_offset = 0;
        ui.add_space(20.);
        let table = TableBuilder::new(ui)
            .striped(false)
            .resizable(false)
            .cell_layout(Layout::left_to_right(Align::Center))
            .column(Column::auto())
            .column(Column::auto())
            .column(Column::auto())
            .column(Column::auto())
            .column(Column::auto())
            .column(Column::auto());

        table.body(|mut body| {
            verb_zeit(
                &mut body,
                T_PRAESENS,
                &verb.praesens,
                viz_level,
                &mut viz_offset,
            );
            verb_zeit(
                &mut body,
                T_PRAETERITUM,
                &verb.praeteritum,
                viz_level,
                &mut viz_offset,
            );
            verb_zeit(
                &mut body,
                T_PERFECT,
                &verb.perfekt,
                viz_level,
                &mut viz_offset,
            );
        });

        Sides::new().show(
            ui,
            |_ui| {},
            |ui| {
                if ui
                    .button(button::next_step(&t!("_next_declination")))
                    .clicked()
                {
                    controller.set_action(crate::ui::Action::NextDeclination);
                }
                if ui.button(button::restart_item()).clicked() {
                    controller.set_action(crate::ui::Action::RestartVerb);
                }
                ui.separator();
                if ui.button(button::next_item(&t!("_next_verb"))).clicked() {
                    controller.set_action(crate::ui::Action::NextVerb);
                }
                ui.separator();
                ui.separator();
                if ui.button(button::cancel()).clicked() {
                    controller.set_action(Action::CloseModal);
                }
            },
        );
    });
}

fn verb_zeit(
    body: &mut TableBody,
    zeit: &str,
    pronomen: &crate::data::Pronomen,
    viz_level: isize,
    viz_offset: &mut isize,
) {
    body.row(40., |mut row| {
        row.col(|ui| {
            ui.label(RichText::new(zeit).heading().strong().underline());
        });
    });
    body.row(25., |mut row| {
        row.col(|ui| {
            verb_zeit_pronomen(ui, "ich", pronomen.ich, viz_level, viz_offset);
        });
        row.col(|ui| {
            verb_zeit_pronomen(ui, "du", pronomen.du, viz_level, viz_offset);
        });
        row.col(|ui| {
            verb_zeit_pronomen(ui, "er/sie/es", pronomen.er_sie_es, viz_level, viz_offset);
        });
        row.col(|ui| {
            verb_zeit_pronomen(ui, "wir", pronomen.wir, viz_level, viz_offset);
        });
        row.col(|ui| {
            verb_zeit_pronomen(ui, "ihr", pronomen.ihr, viz_level, viz_offset);
        });
        row.col(|ui| {
            verb_zeit_pronomen(ui, "sie", pronomen.sie, viz_level, viz_offset);
        });
    });
    body.row(10., |mut row| {
        row.col(|ui| {
            ui.add_space(10.);
        });
    });
    body.row(10., |mut row| {
        row.col(|ui| {
            ui.add_space(10.);
        });
    });
}

fn verb_zeit_pronomen(
    ui: &mut Ui,
    pronom: &str,
    verb: &str,
    viz_level: isize,
    viz_offset: &mut isize,
) {
    let font1 = FontId::new(18., FontFamily::Proportional);
    match viz_level - *viz_offset {
        i if i > 0 => {
            ui.horizontal(|ui| {
                ui.set_width(200.);
                ui.label(RichText::new(pronom).font(font1.clone()));
                ui.label(RichText::new(verb).font(font1).strong());
            });
        }
        0 => {
            ui.horizontal(|ui| {
                ui.label(RichText::new(pronom).font(font1.clone()));
                ui.label(
                    RichText::new("???")
                        .font(font1)
                        .color(Color32::LIGHT_RED)
                        .strong(),
                );
            });
        }
        _ => {}
    }
    *viz_offset += 1;
}
