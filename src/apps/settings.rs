use crate::apps::{CloudShrinkTime, SettingApp, SpeedometerMode, View};
use crate::helpers::Range;
use egui::{Align, Checkbox, Context, Layout, Slider, SelectableLabel};

impl Default for SettingApp {
    fn default() -> Self {
        Self {
            engine_prob_range: Range {
                min: 0.0,
                max: 100.0,
            },
            toggle_200cc: false,
            engine_probs: Default::default(),
            toggle_ct_tt: false,
            toggle_custom_presence: false,
            prevent_selection_online: 0,
            toggle_som: SpeedometerMode::None,
            toggle_drag_blue: false,
            time_cloud: CloudShrinkTime::LECODE as i16,
        }
    }
}

impl View for SettingApp {
    fn name(&self) -> &'static str {
        "LE-CODE Parameters"
    }

    fn ui(&mut self, ctx: &Context) {
        egui::SidePanel::left("lpar_panel")
            .resizable(false)
            .show(ctx, |ui| {
                ui.heading("LPAR Settings");
                ui.add_enabled(false, SelectableLabel::new(self.toggle_200cc == true, "200cc"))
                    .on_disabled_hover_text("200cc is not implemented yet.");
                ui.horizontal(|ui| {
                    ui.label("Engine Probabilities");
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        if ui.button("Reset").clicked() {
                            self.engine_probs = Default::default()
                        }
                    })
                });
                ui.vertical(|ui| {
                    ui.add(
                        Slider::new(
                            &mut self.engine_probs.low,
                            (self.engine_prob_range.min as i32)
                                ..=(self.engine_prob_range.max as i32),
                        )
                        .text(engine_label(false, EngineProbMode::Low)),
                    )
                    .on_hover_text(format!(
                        "Probability for {} online",
                        engine_label(false, EngineProbMode::Low)
                    ));
                    ui.add(
                        Slider::new(
                            &mut self.engine_probs.mid,
                            (self.engine_prob_range.min as i32)
                                ..=(self.engine_prob_range.max as i32),
                        )
                        .text(engine_label(false, EngineProbMode::Mid)),
                    )
                    .on_hover_text(format!(
                        "Probability for {} online",
                        engine_label(false, EngineProbMode::Mid)
                    ));
                    ui.add(
                        Slider::new(
                            &mut self.engine_probs.high,
                            (self.engine_prob_range.min as i32)
                                ..=(self.engine_prob_range.max as i32),
                        )
                        .text(engine_label(false, EngineProbMode::High)),
                    )
                    .on_hover_text(format!(
                        "Probability for {} online",
                        engine_label(false, EngineProbMode::High)
                    ));
                });
                ui.separator();
                ui.horizontal(|ui| {
                    ui.label("CT for Time Trial");
                    ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
                        ui.toggle_value(&mut self.toggle_ct_tt, "Enable").on_hover_text("Allow to select custom tracks on time trial");
                    })
                });
                ui.horizontal(|ui| {
                    ui.label("Extended Presence Flags");
                    ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
                        ui.toggle_value(&mut self.toggle_custom_presence, "Enable")
                        .on_hover_text("Allow to work extended presence flag");
                    })
                });
                ui.label("Block previous tracks");
                ui.add(Slider::new(&mut self.prevent_selection_online, 0..=50)).on_hover_text("Number of races that a previously raced track is blocked\nValus between 0 and 50 are allowed");
                ui.separator();
                ui.label("Speedometer").on_hover_text("Setting for speedometer");
                ui.horizontal_wrapped(|ui| {
                    ui.selectable_value(&mut self.toggle_som, SpeedometerMode::None, "None")
                        .on_hover_text("Hide speedometer from the screen like vanilla");
                    ui.selectable_value(&mut self.toggle_som, SpeedometerMode::Show, "Show")
                        .on_hover_text("Display speedometer at bottom right");
                    ui.selectable_value(&mut self.toggle_som, SpeedometerMode::One, "0.0")
                        .on_hover_text("Display speedometer at bottom right using format 123.4 km/h");
                    ui.selectable_value(&mut self.toggle_som, SpeedometerMode::Two, ".00")
                        .on_hover_text("Display speedometer at bottom right using format 123.45 km/h");
                    ui.selectable_value(&mut self.toggle_som, SpeedometerMode::Three, ".000")
                        .on_hover_text("Display speedometer at bottom right using format 123.456 km/h");
                });
                ui.separator();
                ui.horizontal(|ui| {
                    ui.label("Draggable Blue Shell").on_hover_text("Setting for allowing to drag blue shell");
                    ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
                        ui.toggle_value(&mut self.toggle_drag_blue, "Enable").on_hover_text("Allow player to drag blue shell");
                    });
                });
                ui.horizontal(|ui| {
                    ui.label("Thundercloud Shrink Time");
                    ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
                        ui.text_edit_singleline(&mut "300");
                    })
                })
            });
    }
}

enum EngineProbMode {
    Low,
    Mid,
    High,
}

fn engine_label(custom_engine: bool, mode: EngineProbMode) -> &'static str {
    match mode {
        EngineProbMode::Low => {
            if custom_engine {
                "150cc"
            } else {
                "100cc"
            }
        }
        EngineProbMode::Mid => {
            if custom_engine {
                "200cc"
            } else {
                "150cc"
            }
        }
        _ => "Mirror",
    }
}