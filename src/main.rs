#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![feature(slice_take)]
#![feature(string_remove_matches)]

mod apps;
mod helpers;

use crate::apps::{CheatCodeApp, MainView, SettingApp, TrackDefApp};
use eframe::emath::Align;
use eframe::{App, Frame};
use egui::{Context, Layout};
use std::path::PathBuf;
use std::process::exit;

const APP_NAME: &'static str = "mkw-distro-tool";
const APP_VERSION: &'static str = "v0.1.0";

#[cfg(debug_assertions)]
fn sprint_version() -> String {
    format!("{}-DEBUG", APP_VERSION)
}

#[cfg(not(debug_assertions))]
fn sprint_version() -> String {
    String::from(APP_VERSION)
}

fn main() {
    // let _track_bytes: &[u8] = include_bytes!("../res/tracks.bin") as &[u8];

    let options = eframe::NativeOptions {
        initial_window_size: Some([1280.0, 640.0].into()),
        min_window_size: Some([1280.0, 640.0].into()),
        ..eframe::NativeOptions::default()
    };
    eframe::run_native(APP_NAME, options, Box::new(|_| Box::new(Distro::default())));
}

struct Distro {
    /// To allow displaying closing confirmation
    close_confirm_dialog: bool,
    /// To allow displaying error confirmation.
    /// Messages are determined in Self::err_msg
    confirm_dialog: bool,
    /// (Currently keeps in true) Check the application closable immediately.
    /// Generally the closing confirmation dialog will display if it's true
    disallow_to_close: bool,
    _disallow_to_ignore_change: bool,
    /// Error message
    err_msg: &'static str,
    /// Path to loaded file
    path: Option<PathBuf>,
    //-- Any apps
    /// Track definition
    tracks: TrackDefApp,
    /// LE-PAR definition
    settings: SettingApp,
    /// Cheat code definition
    codes: CheatCodeApp,
}

impl Default for Distro {
    fn default() -> Self {
        Self {
            close_confirm_dialog: false,
            disallow_to_close: false,
            tracks: Default::default(),
            settings: Default::default(),
            codes: Default::default(),
            path: None,
            confirm_dialog: false,
            _disallow_to_ignore_change: false,
            err_msg: "",
        }
    }
}

impl App for Distro {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        frame.set_window_title(&self.gen_title());
        self.view_top_menu(ctx, frame);
        self.settings.ui(ctx);
        self.codes.ui(ctx);
        self.tracks.ui(ctx);

        if self.close_confirm_dialog {
            self.close_confirm(ctx, frame);
        }

        if self.confirm_dialog {
            self.any_confirm(ctx, frame);
        }
    }

    fn on_close_event(&mut self) -> bool {
        self.close_confirm_dialog = true;
        self.disallow_to_close
    }
}

impl Distro {
    fn gen_title(&self) -> String {
        if self.path.is_none() || self.path.as_ref().unwrap().to_str().unwrap() == "" {
            return format!("{}", APP_NAME);
        }

        format!(
            "{} - {}",
            APP_NAME,
            self.path.as_ref().unwrap().to_str().unwrap()
        )
    }

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
                ui.vertical_centered_justified(|ui| {
                    ui.horizontal_wrapped(|ui| {
                        if ui.button("Yes").clicked() {
                            exit(0);
                        }
                        if ui.button("No").clicked() {
                            self.close_confirm_dialog = false;
                        }
                    })
                })
            });
    }

    fn any_confirm(&mut self, ctx: &Context, frame: &mut Frame) {
        let (x, y) = (
            frame.info().window_info.size.x,
            frame.info().window_info.size.y,
        );
        egui::Window::new("Error")
            .title_bar(true)
            .default_width(400.0)
            .collapsible(false)
            .resizable(false)
            .fixed_pos([(x / 2.0) - 200.0, y / 2.5])
            .show(ctx, |ui| {
                ui.label(self.err_msg);
                ui.horizontal(|ui| {
                    if ui.button("Ok").clicked() {
                        self.confirm_dialog = false;
                    }
                })
            });
    }

    fn view_top_menu(&mut self, ctx: &Context, frame: &mut Frame) {
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                egui::widgets::global_dark_light_mode_switch(ui);
                ui.separator();
                ui.menu_button("File", |ui| {
                    if ui.button("New Project").clicked() {
                        self.settings = Default::default();
                        self.tracks = Default::default();
                    }
                    if ui.button("Open Project").clicked() {
                        if let Some(path) = rfd::FileDialog::new()
                            .add_filter(".mkprj", &["mkprj"])
                            .pick_file()
                        {
                            self.open_project(&path);
                        }
                    }
                    ui.separator();
                    if ui.button("Save").clicked() {
                        println!("File:Save");
                        if self.path.is_none() {
                            match rfd::FileDialog::new()
                                .add_filter(".mkprj", &["mkprj"])
                                .save_file()
                            {
                                Some(path) => self.path = Some(path),
                                None => return,
                            }
                        }
                        self.save_project(self.path.as_ref().unwrap());
                    }
                    if ui.button("Save as new").clicked() {
                        match rfd::FileDialog::new()
                            .add_filter(".mkprj", &["mkprj"])
                            .save_file()
                        {
                            Some(path) => self.path = Some(path),
                            None => return,
                        }
                        self.save_project(self.path.as_ref().unwrap());
                    }
                    ui.separator();
                    if ui.button("Exit").clicked() {
                        self.close_confirm_dialog = true;
                    }
                });
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
