use eframe::{egui, WindowAttributes};
use gtfo_codecracker::{load_words_from_str, match_pattern};

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

                if ui
                    .text_edit_singleline(&mut self.pattern)
                    .lost_focus()
                    && ui.input(|i| i.key_pressed(egui::Key::Enter))
                {
                    self.results = match_pattern(&self.pattern, &self.words);
                }
            });

            if ui.button("Crack").clicked() {
                self.results = match_pattern(&self.pattern, &self.words);
            }

            ui.separator();

            if self.results.is_empty() {
                ui.label("No matches yet.");
            } else {
                ui.label("Matching words:");
                for word in &self.results {
                    ui.label(word);
                }
            }
        });
    }
}


fn main() -> Result<(), eframe::Error> {
    let size = egui::vec2(300.0, 800.0);
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size(size),
        ..Default::default()
    };
    
    eframe::run_native(
        "GTFO Code Cracker",
        options,
        Box::new(|_cc| Ok(Box::<CodeCrackerApp>::default())),
    )
}
