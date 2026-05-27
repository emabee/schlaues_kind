use crate::{
    PROG_TITLE, WIN_WIDTH,
    assets::{BURGER_IMG, DICTATE_2, DICTATE_34, LOGO_IMG},
    controller::Controller,
    data::Data,
    ui::{
        Action, modals,
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
        self.top_panel(ctx);

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
                ref mut sound_was_played,
            } => modals::dictate(
                &self.data.dictates[*current_series],
                current_dictate,
                current_line,
                edit_line,
                edit_line_has_focus,
                check_is_on,
                sound_was_played,
                &self.data.sink,
                &mut self.controller,
                ctx,
            ),
        }

        // show the main UI
        Self::main_ui(ctx);
    }
}

impl TheApp<'_> {
    fn top_panel(&mut self, ctx: &Context) {
        TopBottomPanel::top("file").show(ctx, |ui| {
            ui.add_space(2.);
            ui.horizontal(|ui| {
                ui.add_space(ui.available_width() - 100.);
                self.burger_menu_button(ui);
            });
            ui.add_space(2.);
        });
    }

    fn burger_menu_button(&mut self, ui: &mut egui::Ui) {
        MenuBar::new().ui(ui, |ui| {
            ui.menu_image_button(Image::new(BURGER_IMG), |ui| {
                if ui
                    .add_enabled(
                        self.v.modal_state.is_ready_for_modal(),
                        Button::image_and_text(
                            Image::new(LOGO_IMG),
                            format!("{}", t!("About %{name}", name = PROG_TITLE)),
                        ),
                    )
                    .clicked()
                {
                    self.controller.set_action(Action::ShowAbout);
                }

                ui.separator();

                if ui
                    .add_enabled(
                        self.v.modal_state.is_ready_for_modal(),
                        Button::new(format!("{} …", t!("Unregelmässige Verben"))),
                    )
                    .clicked()
                {
                    self.controller.set_action(Action::DeclineVerbs);
                }

                if ui
                    .add_enabled(
                        self.v.modal_state.is_ready_for_modal(),
                        Button::new(format!("{} …", t!("Knifflige Wörter"))),
                    )
                    .clicked()
                {
                    self.controller.set_action(Action::ReadTrickyWords);
                }

                ui.separator();

                if ui
                    .add_enabled(
                        self.v.modal_state.is_ready_for_modal(),
                        Button::new(format!("{} …", t!("_calculate_until_it_smokes"))),
                    )
                    .clicked()
                {
                    self.controller.set_action(Action::ShowMathBasics);
                }

                ui.separator();

                if !self.data.dictates[DICTATE_2].is_empty()
                    && ui
                        .add_enabled(
                            self.v.modal_state.is_ready_for_modal(),
                            Button::new(format!("{} 2 …", t!("_dictate"))),
                        )
                        .clicked()
                {
                    self.controller.set_action(Action::Dictate(DICTATE_2));
                }

                if ui
                    .add_enabled(
                        self.v.modal_state.is_ready_for_modal(),
                        Button::new(format!("{} 3, 4 …", t!("_dictate"))),
                    )
                    .clicked()
                {
                    self.controller.set_action(Action::Dictate(DICTATE_34));
                }

                ui.separator();

                if ui
                    .add_enabled(
                        self.v.modal_state.is_ready_for_modal(),
                        Button::new(format!("{}", t!("Unregelmässige Verben ausdrucken"))),
                    )
                    .clicked()
                {
                    self.controller.set_action(Action::PrintVerbs);
                }
            });
        });
    }

    // show only the enlarged logo
    fn main_ui(ctx: &Context) {
        TopBottomPanel::top("panel_with_tabs").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.add_space(300.);
                ui.add(
                    Image::new(LOGO_IMG)
                        .fit_to_exact_size([WIN_WIDTH, WIN_WIDTH * 0.48].into())
                        .corner_radius(10),
                );
            });
        });
    }
}
