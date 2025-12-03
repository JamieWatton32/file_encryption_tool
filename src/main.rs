mod encrypt;

use crate::encrypt::{decrypt, encrypt};
use eframe::egui;
use std::fs;

fn main() {
    env_logger::init();

    let icon_data = load_icon();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([720.0, 480.0])
            .with_drag_and_drop(true)
            .with_icon(icon_data),
        ..Default::default()
    };
    eframe::run_native(
        "RustProof",
        options,
        Box::new(|cc| {
            setup_custom_theme(&cc.egui_ctx);
            Ok(Box::<RustProof>::default())
        }),
    )
    .unwrap();
}

fn load_icon() -> egui::IconData {
    let icon_bytes = include_bytes!("RustProofIcon.png");
    let image = image::load_from_memory(icon_bytes)
        .expect("Failed to load icon")
        .to_rgba8();
    let (width, height) = image.dimensions();
    let rgba = image.into_raw();

    egui::IconData {
        rgba,
        width,
        height,
    }
}

// Custom Theme
fn setup_custom_theme(ctx: &egui::Context) {
    let mut style = (*ctx.style()).clone();

    // Dark background with slight warm tone (like aged metal)
    style.visuals.window_fill = egui::Color32::from_rgb(25, 22, 20); // Dark warm gray
    style.visuals.panel_fill = egui::Color32::from_rgb(30, 27, 25); // Slightly lighter warm gray

    // Encrypt button - Protective silver/chrome theme
    style.visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(192, 192, 192); // Silver
    style.visuals.widgets.inactive.fg_stroke =
        egui::Stroke::new(1.5, egui::Color32::from_rgb(140, 160, 180));

    style.visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(220, 220, 230); // Bright silver
    style.visuals.widgets.hovered.fg_stroke =
        egui::Stroke::new(2.0, egui::Color32::from_rgb(100, 140, 180));

    style.visuals.widgets.active.bg_fill = egui::Color32::from_rgb(180, 200, 220); // Chrome blue
    style.visuals.widgets.active.fg_stroke =
        egui::Stroke::new(2.5, egui::Color32::from_rgb(60, 120, 180));

    // Text colors - Clean silver for protected content
    style.visuals.override_text_color = Some(egui::Color32::from_rgb(220, 220, 225));

    // Heading/title styling - Metallic silver
    style.visuals.widgets.noninteractive.fg_stroke =
        egui::Stroke::new(1.0, egui::Color32::from_rgb(200, 200, 210));

    // Spacing and sizing
    style.spacing.button_padding = egui::vec2(16.0, 8.0);
    style.spacing.item_spacing = egui::vec2(12.0, 10.0);

    // Apply the custom style
    ctx.set_style(style);
}

// Color constants for themed UI elements
const RUST_ORANGE: egui::Color32 = egui::Color32::from_rgb(183, 65, 14); // Rust/oxidation color
const RUST_BROWN: egui::Color32 = egui::Color32::from_rgb(139, 69, 19); // Corroded metal
const SILVER_SHINE: egui::Color32 = egui::Color32::from_rgb(192, 192, 192); // Protective silver

#[derive(Default)]
struct RustProof {
    dropped_files: Vec<egui::DroppedFile>,
    picked_path: Option<String>,
    passkey: String,
    success: bool,
}

fn read_file_as_bytes(path: &str) -> Option<Vec<u8>> {
    match fs::read(path) {
        Ok(bytes) => Some(bytes),
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            None
        }
    }
}

impl eframe::App for RustProof {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new("Rust")
                        .size(24.0)
                        .color(RUST_ORANGE)
                        .strong(),
                );
                ui.label(
                    egui::RichText::new("Proof")
                        .size(24.0)
                        .color(SILVER_SHINE)
                        .strong(),
                );
            });
            ui.add_space(8.0);
            ui.separator();
            ui.add_space(8.0);
            ui.horizontal(|ui| {
                let width = ui.available_width();
                let file_label_width = 60.0;
                let button_width = 100.0;
                let filename_width = width - file_label_width - button_width - 10.0;

                ui.label("File:");

                if let Some(picked_path) = &self.picked_path {
                    let display_path = if picked_path.len() > 30 {
                        let start = &picked_path[..10];
                        let end = &picked_path[picked_path.len() - 15..];
                        format!("{}…{}", start, end)
                    } else {
                        picked_path.clone()
                    };

                    ui.add_sized(
                        [filename_width, 20.0],
                        egui::Label::new(egui::RichText::new(display_path).monospace()),
                    );
                } else {
                    ui.add_sized(
                        [filename_width, 20.0],
                        egui::Label::new(egui::RichText::new("<None>").monospace()),
                    );
                }
                ui.add_space(10.0);

                if ui
                    .add_sized([button_width, 30.0], egui::Button::new("Open file…"))
                    .clicked()
                    && let Some(path) = rfd::FileDialog::new().pick_file()
                {
                    self.picked_path = Some(path.display().to_string());
                }
            });
            ui.add_space(8.0);

            ui.horizontal(|ui| {
                let width = ui.available_width();
                let label_width = 80.0;
                let textbox_width = width - label_width;

                ui.label("Passkey:");
                ui.add_sized(
                    [textbox_width, 30.0],
                    egui::TextEdit::singleline(&mut self.passkey),
                );
            });

            ui.add_space(10.0);
            ui.horizontal(|ui| {
                let width = ui.available_width();
                let spacing = 10.0; // spacing between buttons
                let btn_width = (width - spacing * 2.0) / 2.0; // equal width for both buttons

                // Encrypt button
                let encrypt_btn = egui::Button::new(
                    egui::RichText::new("🛡 Encrypt (New File)")
                        .color(egui::Color32::from_rgb(40, 40, 50))
                        .strong(),
                )
                .fill(SILVER_SHINE);

                if ui.add_sized([btn_width, 40.0], encrypt_btn).clicked()
                    && let Some(path) = &self.picked_path
                {
                    if let Some(file_bytes) = read_file_as_bytes(path) {
                        match encrypt(None, &file_bytes, self.passkey.as_str()) {
                            Ok(_) => {
                                self.success = true;
                                ui.label("File has been encrypted to encrypted.cocoon");
                            }
                            Err(_) => {
                                self.success = false;
                                ui.label(
                                    "Error occurred when encrypting contents. Please try again.",
                                );
                            }
                        }
                    } else {
                        ui.label("Error reading file");
                    }
                }

                let inline_encrypt_btn = egui::Button::new(
                    egui::RichText::new("🛡 Encrypt (Overwrite)")
                        .color(egui::Color32::from_rgb(40, 40, 50))
                        .strong(),
                )
                .fill(SILVER_SHINE);

                if ui
                    .add_sized([btn_width, 40.0], inline_encrypt_btn)
                    .clicked()
                    && let Some(path) = &self.picked_path
                {
                    if let Some(mut file_bytes) = read_file_as_bytes(path) {
                        match encrypt(Some(path), &mut file_bytes, self.passkey.as_str()) {
                            Ok(_) => {
                                self.success = true;
                                ui.label("File has been encrypted to encrypted.cocoon");
                            }
                            Err(_) => {
                                self.success = false;
                                ui.label(
                                    "Error occurred when encrypting contents. Please try again.",
                                );
                            }
                        }
                    } else {
                        ui.label("Error reading file");
                    }
                }
            });
            ui.add_space(10.0);
            ui.horizontal(|ui| {
                let width = ui.available_width();
                let spacing = 10.0; // spacing between buttons
                let btn_width = (width - spacing * 2.0) / 2.0; // equal width for both buttons
                // Decrypt button
                let decrypt_btn = egui::Button::new(
                    egui::RichText::new("🔓 Decrypt (New File)")
                        .color(egui::Color32::from_rgb(40, 40, 50))
                        .strong(),
                )
                .fill(RUST_BROWN);

                if ui.add_sized([btn_width, 40.0], decrypt_btn).clicked()
                    && let Some(path) = &self.picked_path
                {
                    if let Some(file_bytes) = read_file_as_bytes(path) {
                        match decrypt(None, &file_bytes, self.passkey.as_str()) {
                            Ok(_) => {
                                self.success = true;
                            }
                            Err(_) => {
                                ui.label(
                                    "Error occurred when decrypting contents. Please try again.",
                                );
                                self.success = false;
                            }
                        }
                    } else {
                        ui.label("Error reading file");
                    }
                }

                let inline_decrypt_btn = egui::Button::new(
                    egui::RichText::new("🔓 Decrypt (Restore)")
                        .color(egui::Color32::from_rgb(40, 40, 50))
                        .strong(),
                )
                .fill(RUST_BROWN);

                if ui
                    .add_sized([btn_width, 40.0], inline_decrypt_btn)
                    .clicked()
                    && let Some(path) = &self.picked_path
                {
                    if let Some(file_bytes) = read_file_as_bytes(path) {
                        match decrypt(Some(path), &file_bytes, self.passkey.as_str()) {
                            Ok(_) => {
                                self.success = true;
                            }
                            Err(_) => {
                                ui.label(
                                    "Error occurred when decrypting contents. Please try again.",
                                );
                                self.success = false;
                            }
                        }
                    } else {
                        ui.label("Error reading file");
                    }
                }
            });

            if self.success {
                ui.label("Success");
            }
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
