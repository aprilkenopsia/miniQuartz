//use std::collections::{binary_heap::{IntoIter, Iter}, hash_map::Iter};
use egui::ScrollArea;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    label: String,

    #[serde(skip)] // This how you opt-out of serialization of a field
    value: f32,

    #[serde(skip)] // Opting out of serialization needs this thing above it. What's serde?
    songs: Songs, // I think this is basically doing what the 2021: 7a Rust tutorial said, but fits into the template given by egui. Probably better to do it like this? Feels cleaner.

    #[serde(skip)]
    row_height: Option<f32>,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 2.7,
            // Not example stuff:
            songs: Songs::new(),
            row_height: None,
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.

    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Default::default()
        }
    }
}

struct Songs {
    articles: Vec<SongCardData>
}

struct SongCardData{
    title: String,
    artist: String,
    length: String,
    cover_path: String,
    texture: Option<egui::TextureHandle>,
}

impl Songs{
    fn new() -> Songs{
        let iter = (0..50).map(|a| SongCardData{
            title: (a).to_string(),
            artist: (a+1).to_string(),
            length: (a*2).to_string(),
            cover_path: "assets/icon-256.png".to_owned(),
            texture: None,
        });
        Songs{
            articles: Vec::from_iter(iter)
        }
    }
}

impl SongCardData { //i must be for real this section is written by ai. im Sorry. but im fuck at rust,,
    fn load_texture_if_needed(&mut self, ctx: &egui::Context) {
        if self.texture.is_none() {
            if let Ok(image) = image::open(&self.cover_path) {
                let image = image.to_rgba8();
                let size = [image.width() as usize, image.height() as usize];
                let texture = ctx.load_texture(
                    self.cover_path.clone(), 
                    egui::ColorImage::from_rgba_unmultiplied(size, &image),
                    Default::default()
                );
                self.texture = Some(texture);
            }
        }
    }
}

impl eframe::App for TemplateApp {
    /// Called by the framework to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::MenuBar::new().ui(ui, |ui| {
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

                egui::widgets::global_theme_preference_buttons(ui);
            });
        });

        egui::TopBottomPanel::bottom("status")
        .resizable(true)
        .min_height(100.0)
        .max_height(500.0)
        .show(ctx, |ui|{
            ui.label("Status hereeee!!!");
        });

        egui::SidePanel::left("playlists")
        .resizable(true)
        .min_width(30.0)
        .show(ctx, |ui|{
            ScrollArea::vertical().show(ui, |ui|{
                ui.set_min_width(ui.available_width());
                ui.label("Playlists Hereeeeeeeeeee");
            });
        });


        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("miniQuartz");
            let available_width = ui.available_width();

            ScrollArea::vertical()
            .max_width(available_width)
            .show(ui,|ui|{
                let row_height = self.row_height.unwrap_or(30.0); // proper row height
                let total_rows = self.songs.articles.len();

                let clip_rect = ui.clip_rect();
                let top = clip_rect.top();
                let bottom= clip_rect.bottom();

                let mut start = ((top - ui.min_rect().top()) / row_height).floor() as usize;
                let mut end = ((bottom - ui.min_rect().top()) / row_height).ceil() as usize;

                let buffer_size = 5;

                start = start.saturating_sub(buffer_size);
                end = (end + buffer_size).min(total_rows);

                let above_px = start as f32 * row_height;
                ui.add_space(above_px);

                ui.label(available_width.to_string());

                for i in start..end{ 
                    let song = &mut self.songs.articles[i]; // Ampersand makes it read-only, since the for loop tries to own "articles"
                    song.load_texture_if_needed(ctx);

                    ui.set_min_width(available_width);
                    ui.group(|ui|{
                        ui.horizontal(|ui|{
                            if let Some(tex) = &song.texture {
                                ui.add(
                                egui::Image::new(tex)
                                    .max_width(30.0)
                                    .corner_radius(10),
                            );
                            } else {
                                ui.label("img not found");
                            }

                            ui.vertical(|ui|{
                                ui.label(egui::RichText::new(format!("Title: {}", song.title)).strong());
                                ui.label(format!("Artist {}", song.artist));
                            });
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui|{
                                ui.add_space(10.0);
                                ui.label(format!("Length {}", song.length));
                            });
                        });
                    });  
                }
                let remaining_px = (total_rows - end) as f32 * row_height;
                ui.add_space(remaining_px);
            });
            

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                powered_by_egui_and_eframe(ui);
                egui::warn_if_debug_build(ui);
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
