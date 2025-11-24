mod encrypt;

use eframe::egui;
use std::fs;
use std::io;

fn main() {
    env_logger::init();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([640.0, 240.0]) // wide enough for the drag-drop overlay text
            .with_drag_and_drop(true),
        ..Default::default()
    };
    eframe::run_native(
        "RustProof",
        options,
        Box::new(|_cc| Ok(Box::<RustProof>::default())),
    )
    .unwrap();
}

#[derive(Default)]
struct RustProof {
    dropped_files: Vec<egui::DroppedFile>,
    picked_path: Option<String>,
}

fn read_file_as_bytes(path: &str) -> Option<Vec<u8>> {
    match fs::read(path) {
        Ok(bytes) => {
            println!("Successfully read {} bytes from {}", bytes.len(), path);
            println!("{:#?}", bytes);
            return Some(bytes);
        }
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return None;
        }
    }
}

fn horzontal_label(ui: &mut egui::Ui, static_label: &str, dynamic_label: &String) {
    ui.horizontal(|ui| {
        ui.label(static_label);
        ui.monospace(dynamic_label);
    });
}
impl eframe::App for RustProof {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Encrypt and Decrypt your files");
            ui.spacing();
            ui.horizontal(|ui| {
                if let Some(picked_path) = &self.picked_path {
                    horzontal_label(ui, "File: ", picked_path);
                } else {
                    horzontal_label(ui, "File: ", &String::from("<None>"));
                }
                if ui.button("Open file…").clicked()
                    && let Some(path) = rfd::FileDialog::new().pick_file()
                {
                    self.picked_path = Some(path.display().to_string());
                }
            });
            ui.spacing();
            ui.horizontal(|ui| {
                if ui.button("Encrypt").clicked()
                    && let Some(path) = &self.picked_path
                {
                    if let Some(file_bytes) = read_file_as_bytes(path) {
                        // encrypt(file_bytes);
                    } else {
                        ui.label("Error reading file");
                    }
                }
                if ui.button("Decrypt").clicked()
                    && let Some(path) = &self.picked_path
                {
                    if let Some(file_bytes) = read_file_as_bytes(path) {
                        // decrypt(file_bytes);
                    } else {
                        ui.label("Error reading file");
                    }
                }
            });
        });

        preview_files_being_dropped(ctx);

        // Collect dropped files:
        ctx.input(|i| {
            if !i.raw.dropped_files.is_empty() {
                self.dropped_files.clone_from(&i.raw.dropped_files);
            }
        });
    }
}

/// Preview hovering files:
fn preview_files_being_dropped(ctx: &egui::Context) {
    use egui::{Align2, Color32, Id, LayerId, Order, TextStyle};
    use std::fmt::Write as _;

    if !ctx.input(|i| i.raw.hovered_files.is_empty()) {
        let text = ctx.input(|i| {
            let mut text = "Dropping files:\n".to_owned();
            for file in &i.raw.hovered_files {
                if let Some(path) = &file.path {
                    write!(text, "\n{}", path.display()).ok();
                } else if !file.mime.is_empty() {
                    write!(text, "\n{}", file.mime).ok();
                } else {
                    text += "\n???";
                }
            }
            text
        });

        let painter =
            ctx.layer_painter(LayerId::new(Order::Foreground, Id::new("file_drop_target")));

        let content_rect = ctx.content_rect();
        painter.rect_filled(content_rect, 0.0, Color32::from_black_alpha(192));
        painter.text(
            content_rect.center(),
            Align2::CENTER_CENTER,
            text,
            TextStyle::Heading.resolve(&ctx.style()),
            Color32::WHITE,
        );
    }
}
