mod toggle_switch;

use eframe::{
    egui::{self, CentralPanel, FontFamily, FontId, Grid, TextStyle, Ui},
    NativeOptions,
};
use std::fmt::Write;
use std::path::PathBuf;

#[derive(Debug, PartialEq)]
enum ModelQuality {
    Medium,
}

struct App {
    output_srt_file: bool,
    model: ModelQuality,
    only_output_srt: bool,
    input_file: Option<PathBuf>,
}

impl App {
    fn new(cc: &eframe::CreationContext) -> Self {
        cc.egui_ctx.style_mut(|style| {
            style.text_styles = [
                (
                    TextStyle::Heading,
                    FontId::new(30.0, FontFamily::Proportional),
                ),
                (TextStyle::Body, FontId::new(18.0, FontFamily::Proportional)),
                (
                    TextStyle::Monospace,
                    FontId::new(14.0, FontFamily::Proportional),
                ),
                (
                    TextStyle::Button,
                    FontId::new(14.0, FontFamily::Proportional),
                ),
                (
                    TextStyle::Small,
                    FontId::new(10.0, FontFamily::Proportional),
                ),
            ]
            .into();
        });

        Self {
            output_srt_file: true,
            model: ModelQuality::Medium,
            only_output_srt: true,
            input_file: None,
        }
    }

    fn show_options(&mut self, ui: &mut Ui) {
        ui.label("Output subtitle file:");
        ui.add(toggle_switch::toggle(&mut self.output_srt_file));
        ui.end_row();

        ui.label("Model Quality");
        egui::ComboBox::from_label("")
            .selected_text(format!("{:?}", self.model))
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut self.model, ModelQuality::Medium, "medium");
            });
        ui.end_row();

        ui.label("Only output srt:");
        ui.add(toggle_switch::toggle(&mut self.only_output_srt));
        ui.end_row();

        if ui.button("Open video...").clicked() {
            let path = std::env::current_dir().unwrap();

            self.input_file = rfd::FileDialog::new()
                // .add_filter("text", &["txt", "rs"])
                // .add_filter("rust", &["rs", "toml"])
                .set_directory(&path)
                .pick_file();
        }
        ui.label(format!("{:?}", self.input_file));
        ui.end_row();
    }

    fn get_command(&self) -> String {
        let mut r = String::from("auto_subtitle ");

        // Add model to use
        write!(
            &mut r,
            "--model {} ",
            match self.model {
                ModelQuality::Medium => "medium",
            }
        )
        .unwrap();

        // Add if we want to output srt file
        write!(
            &mut r,
            "--output_srt {} ",
            if self.output_srt_file {
                "true"
            } else {
                "false"
            }
        )
        .unwrap();

        // Add verbose-ness
        r.push_str("--verbose true ");

        // Whether to only output srt file
        write!(
            &mut r,
            "--srt_only {} ",
            if self.only_output_srt {
                "true"
            } else {
                "false"
            }
        )
        .unwrap();

        // Add output directory
        write!(
            &mut r,
            "-o '{}' ",
            if let Some(input_file) = &self.input_file {
                input_file.parent().unwrap().join("out")
            } else {
                PathBuf::new()
            }
            .display()
        )
        .unwrap();

        // Add video
        write!(
            &mut r,
            "'{}'",
            self.input_file.clone().unwrap_or_default().display()
        )
        .unwrap();

        r
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            Grid::new("my_grid")
                .num_columns(2)
                .spacing([40.0, 4.0])
                .striped(true)
                .show(ui, |ui| {
                    self.show_options(ui);
                });

            ui.label(self.get_command());
        });
    }
}

fn main() -> eframe::Result {
    // let window_options = NativeOptions {
    //     viewport: ViewportBuilder::default().with_inner_size([540.0, 960.0]),
    //     ..Default::default()
    // };
    let window_options = NativeOptions::default();

    eframe::run_native(
        "Subtitle UI",
        window_options,
        Box::new(|cc| Ok(Box::new(App::new(cc)))),
    )
}
