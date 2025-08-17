#![windows_subsystem = "windows"]

use eframe::{egui, epaint::image};
use gtfo_codecracker::{load_words_from_str, match_pattern};
use ::image::GenericImageView;    
use ::image::load_from_memory; 
use egui::ScrollArea;



const CSV_DATA: &str = include_str!("../../../gtfo-codecracker/data/gtfo-possible-codes.csv");

pub struct CodeCrackerApp {
    words: Vec<String>,
    pattern: String,
    results: Vec<String>,
}

impl Default for CodeCrackerApp {
    fn default() -> Self {
        let words = load_words_from_str(CSV_DATA).unwrap_or_default();
        Self {
            words,
            pattern: String::new(),
            results: Vec::new(),
        }
    }
}

impl eframe::App for CodeCrackerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("GTFO Code Cracker");

            ui.horizontal(|ui| {
                ui.label("Pattern (use '-' as wildcard):");

                let response = ui.add_sized(
          egui::vec2(ui.spacing().interact_size.x * 0.5, 20.0), // ~4 chars wide
            egui::TextEdit::singleline(&mut self.pattern),
               );
                
                if response.changed() {
                    self.results = match_pattern(&self.pattern, &self.words);
                }

                if ui.button("Crack").clicked() {
                    self.results = match_pattern(&self.pattern, &self.words);
                }

            });

            

            ui.separator();

            if self.results.is_empty() {
                ui.label("No matches yet.");
            } else {
                ui.label("Matching words:");
                ScrollArea::vertical()
                    .max_height(720.0) // set a max height for the scroll area
                    .show(ui, |ui| {
                    for word in &self.results {
                        ui.horizontal(|ui| {
                            ui.label(egui::RichText::new(word).size(20.0));
                            if ui.button("📋 Copy").clicked() {
                                ui.output_mut(|o| o.copied_text = word.clone());
                            }
                        });
                    }
                    });
            }
        });
    }
}



fn main() -> Result<(), eframe::Error> {

    let icon_bytes = include_bytes!("../../../gtfo-codecracker/data/exec-brute-force.png");
    
    let image = load_from_memory(icon_bytes)
        .expect("Failed to load icon")
        .into_rgba8();

    let (width, height) = image.dimensions();

    let icon = egui::IconData {
        rgba: image.into_raw(),
        width,
        height,
    };

    let size = egui::vec2(275.0, 800.0);
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size(size).with_icon(icon),
        ..Default::default()
    };
    
    eframe::run_native(
        "GTFO Code Cracker",
        options,
        Box::new(|_cc| Ok(Box::<CodeCrackerApp>::default())),
    )
}
