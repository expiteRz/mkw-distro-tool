#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod apps;
mod helpers;

use crate::apps::{CheatCodeApp, SettingApp, TrackDefApp, View};
use eframe::emath::Align;
use eframe::{App, Frame};
use egui::{Context, Layout};
use std::process::exit;

const APP_NAME: &'static str = "mkw-distro-tool";
const APP_VERSION: &'static str = "v0.1.0";

#[cfg(debug_assertions)]
fn sprint_version() -> String {
    let version = String::from(APP_VERSION);
    format!("{}-DEBUG", version)
}

#[cfg(not(debug_assertions))]
fn sprint_version() -> String {
    String::from(APP_VERSION)
}

fn main() {
    let options = eframe::NativeOptions {
        initial_window_size: Some([1024.0, 640.0].into()),
        min_window_size: Some([1024.0, 640.0].into()),
        ..eframe::NativeOptions::default()
    };
    eframe::run_native(APP_NAME, options, Box::new(|_| Box::new(Distro::default())));
}

struct Distro {
    close_confirm_dialog: bool,
    allow_to_close: bool,
    //-- Any apps
    tracks: TrackDefApp,
    settings: SettingApp,
    codes: CheatCodeApp,
}

impl Default for Distro {
    fn default() -> Self {
        Self {
            close_confirm_dialog: false,
            allow_to_close: false,
            tracks: Default::default(),
            settings: Default::default(),
            codes: Default::default(),
        }
    }
}

impl App for Distro {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        self.view_top_menu(ctx);
        self.settings.ui(ctx);
        self.codes.ui(ctx);

        /// Temporarily set panel
        /// Later replacing track listing app
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Track Listing");
            apps::tracks::test_view(ui);

            if self.close_confirm_dialog {
                self.close_confirm(ctx, _frame);
            }
        });
    }

    fn on_close_event(&mut self) -> bool {
        self.close_confirm_dialog = true;
        self.allow_to_close
    }
}

impl Distro {
    fn close_confirm(&mut self, ctx: &Context, frame: &mut Frame) {
        let (x, y) = (
            frame.info().window_info.size.x,
            frame.info().window_info.size.y,
        );
        egui::Window::new("Confirm")
            .title_bar(true)
            .default_width(400.0)
            .collapsible(false)
            .resizable(false)
            .fixed_pos([(x / 2.0) - 100.0, y / 2.5])
            .show(ctx, |ui| {
                ui.label("Are you sure to close this application?");
                ui.horizontal_wrapped(|ui| {
                    if ui.button("Yes").clicked() {
                        exit(0);
                    }
                    if ui.button("No").clicked() {
                        self.close_confirm_dialog = false;
                    }
                })
            });
    }

    fn view_top_menu(&mut self, ctx: &Context) {
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                egui::widgets::global_dark_light_mode_switch(ui);
                ui.separator();
                ui.menu_button("File", |ui| {
                    if ui.button("New Project").clicked() {
                        println!("File:New Project");
                        self.settings = Default::default();
                    }
                    if ui.button("Open Project").clicked() {
                        if let Some(path) = rfd::FileDialog::new()
                            .add_filter(".mkdstprj", &["mkdistprj"])
                            .pick_file()
                        {
                            println!("{:?}", path);
                        }
                    }
                    ui.separator();
                    if ui.button("Save").clicked() {
                        println!("File:Save");
                    }
                    if ui.button("Save as new").clicked() {
                        if let Some(dest) = rfd::FileDialog::new()
                            .add_filter(".mkdstprj", &["mkdistprj"])
                            .save_file()
                        {
                            println!("{:?}", dest);
                        }
                    }
                    ui.separator();
                    ui.menu_button("Export", |ui| {
                        ui.menu_button("Track Definition", |ui| {
                            let _ = ui.button("CT-DEF");
                            let _ = ui.button("LE-DEF");
                        });
                        ui.menu_button("Text", |ui| {
                            let _ = ui.button("Export as BMG");
                            let _ = ui.button("Export as text file");
                        });
                        let _ = ui.button("LE-CODE Settings");
                    });
                    ui.menu_button("Import", |ui| {
                        let _ = ui.button("Track Files");
                    });
                    ui.separator();
                    if ui.button("Exit").clicked() {
                        self.close_confirm_dialog = true;
                    }
                });
                ui.menu_button("Build", |ui| {
                    let _ = ui.button("LE-CODE Distribution");
                });
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.label(sprint_version());
                    ui.separator();
                })
            })
        });
    }
}
