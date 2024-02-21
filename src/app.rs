use egui::{Align, Layout};
use log::debug;

use crate::{password_generator, PasswordGenerator};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct MyApp {
    pw_length: usize,

    pw_gen: PasswordGenerator,

    generated_passwords: Vec<String>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            pw_length: 8,
            pw_gen: PasswordGenerator::default(),
            generated_passwords: vec![],
        }
    }
}

impl MyApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for MyApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            debug!("{:?}", ui.available_size());

            // The central panel the region left after adding TopPanel's and SidePanel's
            // ui.heading("password generator");

            ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {
                ui.spacing_mut().item_spacing.x = 14.;
                ui.with_layout(Layout::top_down(Align::LEFT), |ui| {
                    ui.label("std settings:");
                    ui.add(egui::Checkbox::new(
                        &mut self.pw_gen.settings.use_lowercase_letters,
                        "use_lowercase_letters",
                    ));
                    ui.add(egui::Checkbox::new(
                        &mut self.pw_gen.settings.use_uppercase_letters,
                        "use_uppercase_letters",
                    ));
                    ui.add(egui::Checkbox::new(
                        &mut self.pw_gen.settings.use_numbers,
                        "use_numbers",
                    ));
                });
                // ui.separator();
                ui.with_layout(Layout::top_down(Align::LEFT), |ui| {
                    ui.label("special characters:");
                    ui.radio_value(
                        &mut self.pw_gen.settings.special_character_usage,
                        password_generator::SpecialCharacterUsage::None,
                        "None",
                    );
                    ui.radio_value(
                        &mut self.pw_gen.settings.special_character_usage,
                        password_generator::SpecialCharacterUsage::Simple,
                        "Simple",
                    );
                    ui.radio_value(
                        &mut self.pw_gen.settings.special_character_usage,
                        password_generator::SpecialCharacterUsage::All,
                        "All",
                    );
                });
                // ui.separator();
                ui.with_layout(Layout::top_down(Align::LEFT), |ui| {
                    ui.label("length:");
                    ui.add(egui::Slider::new(
                        &mut self.pw_gen.settings.pw_length,
                        1..=48,
                    ));
                });
            });

            if ui.button("Generate").clicked() {
                self.generated_passwords.clear();
                for _ in 0..5 {
                    self.generated_passwords.push(
                        self.pw_gen
                            .generate()
                            .unwrap_or(String::from("error occurred")),
                    )
                }
                debug!(
                    "Generated with len {}: {:?}",
                    self.pw_gen.settings.pw_length, self.generated_passwords
                );
            }

            ui.separator();

            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                for pw in &self.generated_passwords {
                    ui.label(format!("{}", pw));
                }
            });

            ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| {
                ui.separator();
                egui::warn_if_debug_build(ui);
                powered_by_egui_and_eframe(ui);
            });
        });
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}
