use crate::{
    controller::Controller,
    data::{Operation, Operator},
    ui::{Action, SMALL_MODAL_WIDTH, modals::button},
};
use egui::{ComboBox, Context, FontFamily, FontId, Id, Modal, RichText, Sides};
use strum::IntoEnumIterator;

pub fn math_basic_operation(
    operation: &mut Operation,
    selected_operator: &mut Operator,
    show_result: &mut bool,
    controller: &mut Controller,
    ctx: &Context,
) {
    Modal::new(Id::new("math_basic_modal")).show(ctx, |ui| {
        ui.set_width(SMALL_MODAL_WIDTH);

        ui.horizontal(|ui| {
            ui.add_space(ui.available_width() / 2. - 100.);
            ui.label(
                RichText::new(t!("_calculate_until_it_smokes"))
                    .strong()
                    .font(FontId::new(20., FontFamily::Proportional)),
            );
        });

        ui.separator();

        let before = *selected_operator;
        ComboBox::from_id_salt("choose_operator")
            .selected_text(format!("{}", selected_operator.name()))
            .show_ui(ui, |ui| {
                for operator in Operator::iter() {
                    ui.selectable_value(selected_operator, operator, operator.name());
                }
            });
        if *selected_operator != before {
            *operation = Operation::new(*selected_operator);
            *show_result = false;
        }

        ui.add_space(10.);

        ui.horizontal(|ui| {
            ui.label(
                RichText::new(format!(
                    "{v1} {op} {v2} =",
                    v1 = operation.first(),
                    op = operation.symbol(),
                    v2 = operation.second(),
                ))
                .font(FontId::new(30., FontFamily::Proportional)),
            );
            if *show_result {
                ui.label(
                    RichText::new(format!(" {}", operation.result()))
                        .font(FontId::new(30., FontFamily::Proportional)),
                );
            } else {
                ui.label(
                    RichText::new("???")
                        .font(FontId::new(30., FontFamily::Proportional))
                        .color(egui::Color32::DARK_RED),
                );
            }
        });
        ui.separator();

        Sides::new().show(
            ui,
            |_ui| {},
            |ui| {
                if *show_result {
                    if ui
                        .button(button::next_item(&t!("_next_calculation")))
                        .clicked()
                    {
                        controller.set_action(Action::NextCalculation);
                    }
                } else {
                    if ui.button(button::next_step(&t!("_show_result"))).clicked() {
                        controller.set_action(Action::ShowResult);
                    }
                }
                if ui.button(button::cancel()).clicked() {
                    controller.set_action(Action::CloseModal);
                }
            },
        );
    });
}
