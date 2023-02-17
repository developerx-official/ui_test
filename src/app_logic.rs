use eframe::egui;
use eframe::egui::{
    Align2, Color32, Context, Label, RichText, ScrollArea, Separator, Spinner, TextStyle,
    TopBottomPanel, Vec2,
};
use poll_promise::Promise;
use std::thread::sleep;
use std::time::Duration;

#[derive(Default)]
pub(crate) struct MyApp {
    allowed_to_close: bool,
    show_confirmation_dialog: bool,
    load_promise: Option<Promise<()>>,
}

impl eframe::App for MyApp {
    // Update
    fn update(&mut self, ctx: &Context, frame: &mut eframe::Frame) {
        // Init stuff
        self.load_promise.get_or_insert_with(|| {
            Promise::spawn_thread("bg_thread", || {
                sleep(Duration::from_secs(2));
            })
        });

        // Spinner
        match &self.load_promise {
            None => {}
            Some(promise) => match promise.ready() {
                None => {
                    render_spinner(ctx);
                }
                Some(_) => {
                    render_main_body(ctx);
                }
            },
        }

        // Confirmation dialogue
        if self.show_confirmation_dialog {
            egui::Window::new("Quit?")
                .collapsible(false)
                .resizable(false)
                .anchor(
                    Align2::CENTER_CENTER,
                    Into::<Vec2>::into(Vec2::new(0_f32, 0_f32)),
                )
                .default_size(Vec2::new(150_f32, 100_f32))
                .show(ctx, |ui| {
                    ui.columns(2, |ui| {
                        ui[0].vertical_centered_justified(|ui| {
                            if ui.button("No").clicked() {
                                self.show_confirmation_dialog = false;
                            }
                        });
                        ui[1].vertical_centered_justified(|ui| {
                            if ui.button("Yes").clicked() {
                                self.allowed_to_close = true;
                                frame.close();
                            }
                        });
                    });
                });
        }
    }

    fn on_close_event(&mut self) -> bool {
        self.show_confirmation_dialog = true;
        self.allowed_to_close
    }
}

fn render_spinner(ctx: &Context) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.centered_and_justified(|ui| {
            ui.add(Spinner::new().size(40_f32));
        });
    });
    TopBottomPanel::bottom("loading_bottom_panel").show(ctx, |ui| {
        ui.centered_and_justified(|ui| {
            ui.scope(|ui| {
                ui.visuals_mut().override_text_color = Some(Color32::WHITE);
                ui.style_mut().override_text_style = Some(TextStyle::Monospace);
                ui.add(Label::new("Loading..."));
            });
        });
    });
}

fn render_main_body(ctx: &Context) {
    // Bottom
    TopBottomPanel::bottom("bottom_panel")
        .show_separator_line(false)
        .show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add(Separator::default().spacing(0_f32));
                ui.add_space(10_f32);
                ui.label(RichText::new("Made by Developer X").size(15_f32));
                ui.hyperlink_to("Dexeloper", "https://dexeloper.com");
                ui.add_space(10_f32);
            });
        });

    // Main Body
    egui::CentralPanel::default().show(ctx, |ui| {
        // Top
        ui.vertical_centered_justified(|ui| {
            ui.label(RichText::new("App").strong().monospace().size(40_f32));
        });
        ui.add_space(10_f32);
        ui.add(Separator::default().horizontal().spacing(15_f32));

        // Body
        ui.vertical_centered(|ui| {
            ScrollArea::new([false, true]).show(ui, |ui| {
                ui.label(RichText::new("Hello there and welcome to App!\nIn App, we strive to help you do your task!").size(20_f32));
                ui.collapsing(RichText::new("About Us").size(18_f32), |ui| {
                    ui.label(RichText::new("In App, we strive to enable you to do your task!\n\
                   We do this by:\n\
                   a) reason\n\
                   b) reason\n\
                   c) reason")
                        .size(15_f32));
                });
                ui.collapsing(RichText::new("Long paragraph").size(18_f32), |ui| {
                    ui.label(RichText::new("In App, we strive to enable you to do your task!\n\
                   We do this by:\n\
                   a) reason\n\
                   b) reason\n\
                   c) reason\n\
                   d) reason\n\
                   e) reason\n\
                   f) reason\n\
                   g) reason\n\
                   h) reason\n\
                   i) reason")
                        .size(15_f32));
                });
            });
        });
    });
}
