use crate::{
    PROG_TITLE, WIN_WIDTH,
    controller::Controller,
    data::Data,
    ui::{
        Action, IMG_BURGER, IMG_LOGO, modals,
        viz::{ModalState, V},
    },
};
use anyhow::Result;
use eframe::{App, Frame};
use egui::{Button, Context, Image, MenuBar, TopBottomPanel};

// MVC pattern
pub struct TheApp<'a> {
    pub data: Data<'a>,
    pub v: V,
    pub controller: Controller,
}
impl TheApp<'_> {
    pub fn new() -> Result<Self> {
        Ok(TheApp {
            data: Data::new()?,
            v: V::new(),
            controller: Controller::default(),
        })
    }
}

impl App for TheApp<'_> {
    // this method is called each time the UI needs to be updated, which is typically many times per second.
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        // execute action set by the UI code
        self.controller.act(&mut self.data, &mut self.v);

        // render the UI
        top_panel(&mut self.v, &mut self.controller, ctx);

        // show modal if desired
        match self.v.modal_state {
            ModalState::None => {}

            ModalState::About => {
                modals::show_about(&mut self.controller, ctx);
            }

            ModalState::DeclineVerbs {
                verb_idx,
                visibility_level,
            } => {
                modals::verb_declination(
                    verb_idx,
                    visibility_level,
                    &self.data.verbs.verbs,
                    &mut self.controller,
                    ctx,
                );
            }
            ModalState::ReadTrickyWords {
                word_list_index,
                word_idx,
            } => {
                modals::tricky_word(
                    &self.data.word_lists,
                    word_list_index,
                    word_idx,
                    &mut self.controller,
                    ctx,
                );
            }
            ModalState::BasicMath {
                current_operator: ref mut chosen_operator,
                ref mut show_result,
            } => {
                modals::math_basic_operation(
                    &mut self.data.operation,
                    chosen_operator,
                    show_result,
                    &mut self.controller,
                    ctx,
                );
            }
            ModalState::Dictates {
                ref mut current_series,
                ref mut current_dictate,
                ref mut current_line,
                ref mut edit_line,
                ref mut edit_line_has_focus,
                ref mut check_is_on,
            } => modals::dictate(
                if *current_series {
                    &self.data.dictates_2
                } else {
                    &self.data.dictates_34
                },
                current_dictate,
                current_line,
                edit_line,
                edit_line_has_focus,
                check_is_on,
                &mut self.controller,
                ctx,
            ),
        }

        // show the main UI
        main_ui(&mut self.v, &mut self.controller, ctx);
    }
}

pub fn top_panel(v: &mut V, controller: &mut Controller, ctx: &Context) {
    TopBottomPanel::top("file").show(ctx, |ui| {
        ui.add_space(2.);
        ui.horizontal(|ui| {
            ui.add_space(ui.available_width() - 100.);
            burger_menu_button(v, controller, ui);
        });
        ui.add_space(2.);
    });
}

fn burger_menu_button(v: &mut V, controller: &mut Controller, ui: &mut egui::Ui) {
    MenuBar::new().ui(ui, |ui| {
        ui.menu_image_button(Image::new(IMG_BURGER), |ui| {
            if ui
                .add_enabled(
                    v.modal_state.is_ready_for_modal(),
                    Button::image_and_text(
                        Image::new(IMG_LOGO),
                        format!("{}", t!("About %{name}", name = PROG_TITLE)),
                    ),
                )
                .clicked()
            {
                controller.set_action(Action::ShowAbout);
            }

            ui.separator();

            if ui
                .add_enabled(
                    v.modal_state.is_ready_for_modal(),
                    Button::new(format!("{} …", t!("Unregelmässige Verben"))),
                )
                .clicked()
            {
                controller.set_action(Action::DeclineVerbs);
            }

            if ui
                .add_enabled(
                    v.modal_state.is_ready_for_modal(),
                    Button::new(format!("{} …", t!("Knifflige Wörter"))),
                )
                .clicked()
            {
                controller.set_action(Action::ReadTrickyWords);
            }

            ui.separator();

            if ui
                .add_enabled(
                    v.modal_state.is_ready_for_modal(),
                    Button::new(format!("{} …", t!("_calculate_until_it_smokes"))),
                )
                .clicked()
            {
                controller.set_action(Action::ShowMathBasics);
            }

            ui.separator();

            if ui
                .add_enabled(
                    v.modal_state.is_ready_for_modal(),
                    Button::new(format!("{} 2 …", t!("_dictate"))),
                )
                .clicked()
            {
                controller.set_action(Action::Dictate(false));
            }

            if ui
                .add_enabled(
                    v.modal_state.is_ready_for_modal(),
                    Button::new(format!("{} 3, 4 …", t!("_dictate"))),
                )
                .clicked()
            {
                controller.set_action(Action::Dictate(true));
            }

            ui.separator();

            if ui
                .add_enabled(
                    v.modal_state.is_ready_for_modal(),
                    Button::new(format!("{}", t!("Unregelmässige Verben ausdrucken"))),
                )
                .clicked()
            {
                controller.set_action(Action::PrintVerbs);
            }
        });
    });
}

pub fn main_ui(_v: &mut V, _controller: &mut Controller, ctx: &Context) {
    // show logo here
    TopBottomPanel::top("panel_with_tabs").show(ctx, |ui| {
        ui.horizontal(|ui| {
            ui.add_space(300.);
            ui.add(
                Image::new(IMG_LOGO)
                    .fit_to_exact_size([WIN_WIDTH, WIN_WIDTH * 0.48].into())
                    .corner_radius(10),
            );
        });
    });
}
