mod encrypt;

use eframe::egui;
use std::fs;

fn main() {
    env_logger::init();
    
    let icon_data = load_icon();
    
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([440.0, 240.0]) 
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
    style.visuals.panel_fill = egui::Color32::from_rgb(30, 27, 25);  // Slightly lighter warm gray
    
    // Encrypt button - Protective silver/chrome theme
    style.visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(192, 192, 192); // Silver
    style.visuals.widgets.inactive.fg_stroke = egui::Stroke::new(1.5, egui::Color32::from_rgb(140, 160, 180));
    
    style.visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(220, 220, 230); // Bright silver
    style.visuals.widgets.hovered.fg_stroke = egui::Stroke::new(2.0, egui::Color32::from_rgb(100, 140, 180));
    
    style.visuals.widgets.active.bg_fill = egui::Color32::from_rgb(180, 200, 220); // Chrome blue
    style.visuals.widgets.active.fg_stroke = egui::Stroke::new(2.5, egui::Color32::from_rgb(60, 120, 180));
    
    // Text colors - Clean silver for protected content
    style.visuals.override_text_color = Some(egui::Color32::from_rgb(220, 220, 225));
    
    // Heading/title styling - Metallic silver
    style.visuals.widgets.noninteractive.fg_stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(200, 200, 210));
    
    // Spacing and sizing
    style.spacing.button_padding = egui::vec2(16.0, 8.0);
    style.spacing.item_spacing = egui::vec2(12.0, 10.0);
    
    // Apply the custom style
    ctx.set_style(style);
}

// Color constants for themed UI elements
const RUST_ORANGE: egui::Color32 = egui::Color32::from_rgb(183, 65, 14);  // Rust/oxidation color
const RUST_BROWN: egui::Color32 = egui::Color32::from_rgb(139, 69, 19);   // Corroded metal
const SILVER_SHINE: egui::Color32 = egui::Color32::from_rgb(192, 192, 192); // Protective silver
const CHROME_BLUE: egui::Color32 = egui::Color32::from_rgb(176, 196, 222); // Clean chrome

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

fn horzontal_label(ui: &mut egui::Ui, static_label: &str, dynamic_label: &String, max_width: f32) {
    ui.horizontal(|ui| {
        ui.label(static_label);
             ui.monospace(dynamic_label);
    });
}
impl eframe::App for RustProof {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("Rust")
                    .size(24.0)
                    .color(RUST_ORANGE)
                    .strong());
                ui.label(egui::RichText::new("Proof")
                    .size(24.0)
                    .color(SILVER_SHINE)
                    .strong());
            });
            ui.add_space(8.0);
            ui.separator();
            ui.add_space(8.0);
            ui.horizontal(|ui| {
                if let Some(picked_path) = &self.picked_path {
                    horzontal_label(ui, "File: ", picked_path, 400.0);
                } else {
                    horzontal_label(ui, "File: ", &String::from("<None>"), 400.0);
                }
                if ui.button("Open file…").clicked()
                    && let Some(path) = rfd::FileDialog::new().pick_file()
                {
                    self.picked_path = Some(path.display().to_string());
                }
            });
            ui.add_space(8.0);
            ui.horizontal(|ui| {
                let encrypt_btn = egui::Button::new(
                    egui::RichText::new("🛡 Encrypt (Protect)")
                        .color(egui::Color32::from_rgb(40, 40, 50))
                        .strong()
                ).fill(SILVER_SHINE);
                
                if ui.add(encrypt_btn).clicked()
                    && let Some(path) = &self.picked_path
                {
                    if let Some(file_bytes) = read_file_as_bytes(path) {
                        // encrypt(file_bytes);
                    } else {
                        ui.label("Error reading file");
                    }
                }
                
                ui.add_space(10.0);
                
                let decrypt_btn = egui::Button::new(
                    egui::RichText::new("🔓 Decrypt (Restore)")
                        .color(egui::Color32::from_rgb(40, 40, 50))
                        .strong()
                ).fill(RUST_BROWN);
                
                if ui.add(decrypt_btn).clicked()
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
