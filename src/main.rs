#![windows_subsystem = "windows"]
use crate::TileType::*;
use eframe::egui;
use eframe::egui::color_picker::{color_picker_color32, Alpha};
use eframe::egui::{vec2, Color32, TextEdit, Vec2, ViewportBuilder, Widget};
use ::image::{GenericImageView, ImageBuffer, Rgba};
use image::ImageFormat;
use rand::Rng;
use std::fs::create_dir;
use std::io::Cursor;
use std::ops::RangeInclusive;
use std::path::PathBuf;
use std::string::ToString;

#[cfg(not(target_arch = "wasm32"))]
fn main(){
    let mut native_options = eframe::NativeOptions::default();
    let custom_viewport = ViewportBuilder::default()
        .with_resizable(false)
        .with_maximize_button(false)
        .with_max_inner_size(Vec2::new(510.0, 410.0));
    native_options.viewport = custom_viewport;

    eframe::run_native("Terraria Tile Generator", native_options, Box::new(|cc| Ok(Box::new(TTGGui::new(cc))))).expect("TODO: panic message");
}

fn make_tile(texture_name: &String, texture_type: &TileType, shades_number: u8, colour: &Color32) {
    // Load the image
    let smooth_bytes = include_bytes!("../resources/TTGS.png");
    let rough_bytes = include_bytes!("../resources/TTG.png");

    let path = "Blocks".to_string();
    if !PathBuf::from(&path).exists() {
        create_dir(path).unwrap();
        println!("Created \"Blocks\" dir");
    }

    let img;
    match texture_type {
        SMOOTH => {
            img = image::load(Cursor::new(smooth_bytes),ImageFormat::Png).unwrap();
        }
        ROUGH => {
            img = image::load(Cursor::new(rough_bytes),ImageFormat::Png).unwrap();
        }
    }

    let (width, height) = img.dimensions();

    let colours = colour_to_arr(&colour);

    // Create a new image buffer
    let mut img_buf = ImageBuffer::new(width, height);

    // Generate colors
    let mut colors = vec![colours.clone()];
    for i in 1..=shades_number {
        let dark_shade = Rgba([
            colours[0].saturating_sub(10 * i).min(255),
            colours[1].saturating_sub(10 * i).min(255),
            colours[2].saturating_sub(10 * i).min(255),
            255,
        ]);
        let light_shade = Rgba([
            colours[0].saturating_add(10 * i).min(255),
            colours[1].saturating_add(10 * i).min(255),
            colours[2].saturating_add(10 * i).min(255),
            255,
        ]);
        colors.push(light_shade);
        colors.push(dark_shade);
    }

    // Randomly generate the tile colors
    let mut rng = rand::thread_rng();
    for (x, y, pixel) in img_buf.enumerate_pixels_mut() {
        if img.get_pixel(x, y)[1] == 255 { // Assuming a specific condition for the pixel
            let random_color = colors[rng.gen_range(0..colors.len())];
            *pixel = random_color;
        } else {
            *pixel = img.get_pixel(x, y);
        }
    }

    // Save the generated image
    match img_buf.save(format!("Blocks/{}.png", &texture_name)) {
        Ok(_) => {println!("Created and saved image!")}
        Err(_) => {println!("Failed to save image.")}
    };
}

fn colour_to_arr(colour: &Color32) -> Rgba<u8> {
    Rgba([colour.r(), colour.g(), colour.b(), 255])
}

#[derive(Default, PartialEq, Copy, Clone)]
enum TileType {
    SMOOTH,
    #[default]
    ROUGH
}

#[derive(Default)]
struct TTGGui {
    colour: Color32,
    texture_name: String,
    texture_type: TileType,
    shades_number: u8
}

impl TTGGui {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_pixels_per_point(1.15);
        let colour = Color32::from_rgb(255, 0, 0);
        let texture_name = String::from("My Awesome Tile");
        let texture_type = ROUGH;
        let shades_number = 1;

        Self {
            colour,
            texture_name,
            texture_type,
            shades_number
        }
    }
}

impl eframe::App for TTGGui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, | ui| {
            ui.vertical_centered(|ui| {
                ui.heading("Terraria Tile Generator");
            });
        });
        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, | ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(1.0);
                if ui.button("Generate!").clicked() {
                    make_tile(&self.texture_name, &self.texture_type, self.shades_number.clone(), &self.colour)
                };
            });
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::Grid::new("main").min_row_height(30.0).spacing([10.0, 10.0]).show(ui, |ui| {
                ui.label("Pick a base color:");
                color_picker_color32(ui, &mut self.colour, Alpha::Opaque);
                ui.end_row();

                ui.label("Enter Texture name:");
                ui.add_sized(vec2(210.0, 15.0), TextEdit::singleline(&mut self.texture_name));
                ui.end_row();

                ui.label("Number of lighter and darker shades:");
                ui.add(egui::DragValue::new(&mut self.shades_number)
                    .range(RangeInclusive::new(1, 25)));
                ui.end_row();

                ui.label("Select Texture Type:");
                ui.vertical(|ui| {
                    ui.selectable_value(&mut self.texture_type, ROUGH, "Rough Type");
                    ui.selectable_value(&mut self.texture_type, SMOOTH, "Smooth Type");
                });
            });
        });
    }
}

