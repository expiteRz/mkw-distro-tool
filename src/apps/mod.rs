use crate::apps::tracks::TrackDefinition;
use egui::Context;

pub mod codes;
pub mod settings;
pub mod tracks;

#[derive(PartialEq)]
pub enum Region {
    NTSC,
    PAL,
    JAP,
    KOR,
}

#[derive(Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct TrackDefApp {
    pub editor: TrackDefinition,
}

#[derive(Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct State {
    track: TrackDefApp,
}

#[derive(PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct SettingApp {
    // ENABLE-200CC
    pub toggle_200cc: bool,
    // ENGINE
    pub engine_probs: EngineProbSet,
    // PERF-MONITOR
    // pub toggle_perf: bool,
    // CUSTOM-TT
    pub toggle_ct_tt: bool,
    // XPFLAGS
    pub toggle_custom_presence: bool,
    // BLOCK-TRACK
    pub prevent_selection_online: i8,
    // SPEEDOMETER
    pub toggle_som: SpeedometerMode,
    // DRAG-BLUE-SHELL
    pub toggle_drag_blue: bool,
    // THCLOUD-TIME
    pub time_cloud: i16,
}

#[derive(PartialEq)]
pub enum SpeedometerMode {
    None,
    Show,
    One,
    Two,
    Three,
}

#[derive(PartialEq)]
pub enum CloudShrinkTime {
    Nintendo = 612,
    LECODE = 300,
}

#[derive(PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct CheatCodeApp {
    pub enabled: bool,
    pub selected_code: usize,
    pub selected_region: Region,
    pub codes: Box<Vec<CodeStruct>>,
    pub vertical_scroll_offset: Option<f32>,
}

#[derive(PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct CodeStruct {
    name: String,
    code_ntsc: String,
    code_pal: String,
    code_jp: String,
    code_kor: String,
}

impl Default for CodeStruct {
    fn default() -> Self {
        CodeStruct {
            name: String::from(""),
            code_ntsc: String::from(""),
            code_pal: String::from(""),
            code_jp: String::from(""),
            code_kor: String::from(""),
        }
    }
}

#[derive(PartialEq)]
pub struct EngineProbSet {
    pub low: i32,
    pub mid: i32,
    pub high: i32,
}

impl Default for EngineProbSet {
    fn default() -> Self {
        Self { low: 10, mid: 60, high: 30 }
    }
}

pub trait View {
    fn name(&self) -> &'static str;
    fn ui(&mut self, ctx: &Context);
}

impl CheatCodeApp {
    pub(crate) fn enabled(&self) -> bool {
        self.enabled
    }
}
