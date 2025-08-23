#![windows_subsystem = "windows"]

use eframe::{egui::{self, scroll_area::ScrollSource, Color32, RichText, Vec2}};
use egui::ScrollArea;
use gtfo_codecracker::{load_words_from_str, match_pattern, blend_color};
// use rdev::{listen, EventType, Key};

const LETTER_SIZE: f32 = 20.0;
const SCROLL_HEIGHT: f32 = 720.0;

const UI_SIZE_EXPANDED: eframe::egui::Vec2 = Vec2::new(250.0, 800.0);
const UI_SIZE_COLLAPSED: eframe::egui::Vec2 = Vec2::new(64.0, 64.0);

const CSV_DATA: &str = include_str!("../../../gtfo-codecracker/data/gtfo-possible-codes.csv");

/* -------------------------------------------------------------------------- */
/*                        GUI CODE BY OSKAR 17-08-2025                        */
/* -------------------------------------------------------------------------- */

pub struct CodeCrackerApp {
    words: Vec<String>,
    pattern: String,
    results: Vec<String>,
    icon: Option<egui::TextureHandle>,
}

impl Default for CodeCrackerApp {
    fn default() -> Self {
        let words = load_words_from_str(CSV_DATA).unwrap_or_default();
        Self {
            words,
            pattern: String::new(),
            results: Vec::new(),
            icon: None,
        }
    }
}

impl eframe::App for CodeCrackerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let focused = ctx.input(|i| i.viewport().focused);
        
        if self.pattern.trim().is_empty(){
            self.results.clear();
        }

        if focused.unwrap_or_default() || !self.results.is_empty() {
            ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(UI_SIZE_EXPANDED));
            egui::CentralPanel::default().show(ctx, |ui| {
                let scroll_source = ScrollSource {
                    scroll_bar: true,
                    drag: false,
                    mouse_wheel: true,
                };

                

                let mut color: Color32 = if !self.results.is_empty() {
                    blend_color(self.results.iter().count())
                } else if self.results.is_empty() {
                    egui::Color32::RED
                } else {
                    egui::Color32::BROWN
                };

                if self.pattern.is_empty() {
                    color = egui::Color32::LIGHT_BLUE;
                }
                
                ui.heading(RichText::new("GTFO Code Cracker").size(25.0).color(color));

                ui.horizontal(|ui| {
                    ui.label(RichText::new("Pattern (e.g. a--e):").size(22.0));

                    let response = ui.add_sized(
                        egui::vec2(ui.spacing().interact_size.x * 1.0, 30.0), // ~4 chars wide
                        egui::TextEdit::singleline(&mut self.pattern)
                        .font(egui::TextStyle::Heading)
                        .horizontal_align(egui::Align::Center)
                        .char_limit(4)
                        .vertical_align(egui::Align::Center),
                    );

                    if response.changed() {
                        self.results = match_pattern(&self.pattern, &self.words);
                    }
                });

                ui.separator();

                if self.results.is_empty() {
                    ui.label(RichText::new("No matching words.").size(15.0));
                    ui.separator().highlight();
                } else {
                    ui.label(RichText::new(format!("Matching words: {}", self.results.iter().count())).size(LETTER_SIZE).color(egui::Color32::GOLD));
                    ui.separator().highlight();
                    ScrollArea::vertical()
                        .max_height(SCROLL_HEIGHT)
                        .auto_shrink(false)
                        .scroll_source(scroll_source)
                        .show(ui, |ui| {
                            for word in &self.results {
                                ui.horizontal(|ui| {
                                    ui.label(
                                        egui::RichText::new(word).size(LETTER_SIZE).monospace(),
                                    );
                                    ui.add_space(ui.available_width() - 10.0);  // Dont change it will not work
                                    ui.add_space(-60.0);                        // Dont change it will not work
                                    if ui.button("📋 Copy").clicked() {
                                        ui.ctx().copy_text(word.clone());
                                    }
                                });
                                ui.separator();
                            }
                        });
                }
            });
        } else {
            ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(UI_SIZE_COLLAPSED));
            ctx.send_viewport_cmd(egui::ViewportCommand::Transparent(true));

            egui::CentralPanel::default().show(ctx, |ui| {
                ui.centered_and_justified(|ui| {
                    if let Some(icon) = &self.icon {
                        ui.image(icon); // use &TextureHandle
                    }
                });
            });
        }

        /* -------------------------------------------------------------------------- */
        /*             Window Focus doesn't work yet. But im still trying             */
        /* -------------------------------------------------------------------------- */

        // if ctx.input(|i| i.key_pressed(egui::Key::F)) {
        //     ctx.send_viewport_cmd(egui::ViewportCommand::Focus);
        // }


    }
}

fn main() -> Result<(), eframe::Error> {
    let icon_bytes = include_bytes!("../../../gtfo-codecracker/data/exec-brute-force.png");
    let image = image::load_from_memory(icon_bytes)
        .expect("Failed to load icon")
        .into_rgba8();

    let (width, height) = image.dimensions();
    let pixels = image.clone().into_raw(); // clone before consuming

    let icon = egui::IconData {
        rgba: pixels.clone(), // this can be reused
        width,
        height,
    };

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size(UI_SIZE_EXPANDED)
            .with_icon(icon)
            .with_resizable(false)
            .with_always_on_top()
            .with_min_inner_size(UI_SIZE_COLLAPSED),
        ..Default::default()
    };

    eframe::run_native(
        "",
        options,
        Box::new(move |cc| {
            
            let size = [width as usize, height as usize];
            let texture = cc.egui_ctx.load_texture(
                "app_icon",
                egui::ColorImage::from_rgba_unmultiplied(size, &pixels),
                Default::default(),
            );

            Ok(Box::new(CodeCrackerApp {
                words: load_words_from_str(CSV_DATA).unwrap_or_default(),
                pattern: String::new(),
                results: Vec::new(),
                icon: Some(texture),
            }))
        }),
    )
}
