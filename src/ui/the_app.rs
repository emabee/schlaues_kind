use crate::{
    PROG_TITLE, WIN_WIDTH,
    controller::Controller,
    data::Data,
    ui::{
        Action, IMG_BURGER, IMG_LOGO,
        modals::{math_basic_operation, show_about, tricky_word, verb_declination},
        viz::{ModalState, V},
    },
};
use anyhow::Result;
use eframe::{App, Frame};
use egui::{Button, Context, Image, MenuBar, TopBottomPanel};

// MVC pattern
pub struct TheApp {
    pub data: Data,
    pub v: V,
    pub controller: Controller,
}
impl TheApp {
    pub fn new() -> Result<Self> {
        Ok(TheApp {
            data: Data::new()?,
            v: V::new()?,
            controller: Controller::default(),
        })
    }
}

impl App for TheApp {
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
                show_about(&mut self.controller, ctx);
            }

            ModalState::DeclineVerbs {
                verb_idx,
                visibility_level,
            } => {
                verb_declination(
                    verb_idx,
                    visibility_level,
                    &self.data.verbs,
                    &mut self.controller,
                    ctx,
                );
            }
            ModalState::ReadTrickyWords {
                ref mut word_idx,
                selected_category: ref mut selected_tricky_word_category,
            } => {
                tricky_word(
                    word_idx,
                    &self.data.tricky_short_words,
                    &self.data.tricky_words,
                    &self.data.tricky_long_words,
                    selected_tricky_word_category,
                    &mut self.controller,
                    ctx,
                );
            }
            ModalState::BasicMath {
                ref mut chosen_operator,
                ref mut show_result,
            } => {
                math_basic_operation(
                    &mut self.data.operation,
                    chosen_operator,
                    show_result,
                    &mut self.controller,
                    ctx,
                );
            }
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
