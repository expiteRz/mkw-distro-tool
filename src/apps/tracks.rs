use std::vec;

use egui::{Align, Layout};
use egui_extras::{RetainedImage, Size, TableBuilder};

use super::{ElementView, MainView, TrackDefApp};

pub struct TrackDefinition {
    pub mode: CupSettings,
    pub selected: usize,
    pub cups: Vec<Cup>,
}

pub struct CupSettings {
    // They could be an enum?
    pub nintendo: bool,
    pub nin_swap: bool,
    pub wiimm_cup: bool,
}

pub struct Cup {
    /// path to image
    pub icon: Icon,
    pub name: String,
    pub trackset: Vec<Track>,
}

#[derive(Debug, Clone)]
pub struct Icon {
    pub filename: String,
    pub image: Vec<u8>,
}

impl Default for Icon {
    fn default() -> Self {
        Self {
            filename: Default::default(),
            image: Default::default(),
        }
    }
}

pub struct Track {
    // Doesn't affect to LE-BIN
    pub id: usize,
    pub name: String,
    pub author: String,
    pub property: Id,
    pub music: Id,
    pub new: bool,
    pub flag: GroupFlag,
    pub filename: String,
}

pub enum GroupFlag {
    None,
    Header,
    Child,
}

impl TrackDefinition {
    fn gen_tables(&mut self, ui: &mut egui::Ui) {
        let table = TableBuilder::new(ui)
            .striped(true)
            .cell_layout(Layout::left_to_right(Align::Center))
            .column(Size::remainder().at_least(60.0));

        table.body(|mut body| {
            self.cups.iter_mut().enumerate().for_each(|(i, x)| {
                body.row(17.0, |mut row| {
                    row.col(|ui| {
                        if ui
                            .add_sized(ui.available_size(), egui::SelectableLabel::new(i == self.selected, &x.name))
                            .clicked()
                        {
                            self.selected = i;
                        }
                    });
                })
            })
        })
    }
}

impl MainView for TrackDefApp {
    fn name(&self) -> &'static str {
        "Track Listing"
    }

    fn ui(&mut self, ctx: &egui::Context) {
        egui::SidePanel::right("cup_list")
            .min_width(200.0)
            .resizable(false)
            .show(ctx, |ui| {
                ui.heading("Cup Listing");
                ui.horizontal(|ui| {
                    egui::ComboBox::from_id_source("cup_special_settings")
                        .selected_text("Cup Setup")
                        .show_ui(ui, |ui| {
                            ui.toggle_value(&mut self.editor.mode.nintendo, "Nintendo Cups")
                                .on_hover_text("Allow to add Nintendo track cups");
                            ui.add_enabled_ui(self.editor.mode.nintendo, |ui| {
                                ui.toggle_value(&mut self.editor.mode.nin_swap, "Swap Nintendo Cups")
                                    .on_hover_text("When the Nintendo cups added, reorder the cups to look the same as vanilla.");
                            });
                            ui.toggle_value(&mut self.editor.mode.wiimm_cup, "Wiimm Cup")
                                .on_hover_text("Allow to add the randomize cup");
                        });
                    ui.with_layout(Layout::right_to_left(egui::Align::Min), |ui| {
                        if ui
                            .button("Delete")
                            .on_hover_text("Delete the selected cup")
                            .clicked()
                        {
                            println!("Cup:Delete");
                            if self.editor.cups.len() <= 1 {
                                return;
                            }
                            let selected = self.editor.selected;
                            self.editor.selected = if selected < 1 {
                                self.editor.selected
                            } else {
                                self.editor.selected - 1
                            };
                            self.editor.cups.remove(selected);
                        }
                        if ui.button("Add").on_hover_text("Add a new cup").clicked() {
                            self.editor.cups.push(Cup::default(self.editor.cups.len()));
                        }
                    });
                });
                ui.separator();
                self.editor.gen_tables(ui);
            });
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(self.name());
            self.editor.cups[self.editor.selected].view(ctx, ui);
        });
    }
}

impl Default for TrackDefinition {
    fn default() -> Self {
        Self {
            selected: 0,
            cups: vec![Cup::default(0)],
            mode: CupSettings {
                nintendo: true,
                nin_swap: true,
                wiimm_cup: true,
            },
        }
    }
}

impl Cup {
    fn default(n: usize) -> Self {
        Self {
            icon: Default::default(),
            trackset: vec![
                Track {
                    id: (n * 4),
                    ..Default::default()
                },
                Track {
                    id: (n * 4) + 1,
                    ..Default::default()
                },
                Track {
                    id: (n * 4) + 2,
                    ..Default::default()
                },
                Track {
                    id: (n * 4) + 3,
                    ..Default::default()
                },
            ],
            name: format!("Cup {}", n),
        }
    }
}

impl ElementView for Cup {
    fn view(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        let texture = if self.icon.filename.is_empty() && self.icon.image.is_empty() {
            RetainedImage::from_color_image("None.png", egui::ColorImage::from_rgba_unmultiplied([1; 2], &[0, 0, 0, 0]))
        } else {
            RetainedImage::from_image_bytes(&self.icon.filename, &self.icon.image).unwrap()
        };

        ui.group(|ui| {
            ui.horizontal(|ui| {
                if ui
                    .add(egui::ImageButton::new(texture.texture_id(ctx), [78.0, 78.0]))
                    .on_hover_text("Select a image to set as cup icon")
                    .context_menu(|ui| {
                        if ui.button("Select a image").clicked() {
                            self.open_image();
                        }
                        if ui.button("Remove image").clicked() {
                            self.icon.filename = "".to_string();
                            self.icon.image = vec![];
                        }
                    })
                    .clicked()
                {
                    self.open_image();
                }
                ui.add(
                    egui::TextEdit::singleline(&mut self.name)
                        .desired_width(300.0)
                        .hint_text("Cup name"),
                );
            });
            for track in &mut self.trackset {
                track.view(ctx, ui);
            }
        });
    }
}

impl Default for Track {
    fn default() -> Self {
        Self {
            id: 0,
            name: "".to_string(),
            property: Id::MarioCircuit,
            music: Id::MarioCircuit,
            flag: GroupFlag::None,
            filename: "".to_string(),
            author: "".to_string(),
            new: false,
        }
    }
}

impl ElementView for Track {
    fn view(&mut self, _ctx: &egui::Context, ui: &mut egui::Ui) {
        ui.group(|ui| {
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.add(egui::TextEdit::singleline(&mut self.name).hint_text("Track name"));
                    ui.checkbox(&mut self.new, "New").on_hover_text(format!("Mark as new track\nIt will include in the option {} in Wiimm Cup", r#""Random: New Track""#));
                });
                ui.add(egui::TextEdit::singleline(&mut self.author).hint_text("Author name"));
                ui.with_layout(Layout::left_to_right(egui::Align::Min), |ui| {
                    egui::ComboBox::new(format!("property_{}", &mut self.id), "Property")
                        .width(180.0)
                        .selected_text(format!("{}", &mut self.property.as_str()))
                        .show_ui(ui, |ui| {
                            for v in Id::VALUES {
                                if v == Id::GalaxyArena {
                                    continue;
                                }
                                ui.selectable_value(&mut self.property, v, v.as_str());
                            }
                        });
                    egui::ComboBox::new(format!("music_{}", &mut self.id), "Music")
                        .width(180.0)
                        .selected_text(format!("{}", &mut self.music.as_str()))
                        .show_ui(ui, |ui| {
                            for v in Id::VALUES {
                                ui.selectable_value(&mut self.music, v, v.as_str());
                            }
                        });
                });
                ui.separator();
                ui.horizontal(|ui| {
                    ui.add(
                        egui::TextEdit::singleline(&mut self.filename)
                            .hint_text("Path to track file (*.szs)")
                            .desired_width(400.0),
                    );
                    if ui
                        .button("...")
                        .on_hover_text("Select a SZS file")
                        .clicked()
                    {
                        self.open_file();
                    }
                });
            });
        });
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum Id {
    LuigiCircuit,
    MooMooMeadow,
    MushroomGorge,
    ToadsFactory,
    MarioCircuit,
    CoconutMall,
    DKSummit,
    WariosGoldMine,
    DaisyCircuit,
    KoopaCape,
    MapleTreeway,
    GrumbleVolcano,
    DryDryRuins,
    MoonviewHighway,
    BowsersCastle,
    RainbowRoad,
    PeachBeachGCN,
    YoshiFallsDS,
    GhostValleySNES,
    MarioRacewayN64,
    SherbetLandN64,
    ShyGuyBeachGBA,
    DelfinoSquareDS,
    WaluigiStadiumGCN,
    DesertHillsDS,
    BowserCastleGBA,
    JungleParkwayN64,
    MarioCircuitGCN,
    MarioCircuitSNES,
    PeachGardenDS,
    DKMountainGCN,
    BowserCastleN64,
    BlockPlaza,
    DelfinoPier,
    FunkyStadium,
    ChompWheel,
    ThwompDesert,
    BattleCourseSNES,
    BattleCourseGBA,
    SkyscraperN64,
    CookieLandGCN,
    TwilightHouseDS,
    // For only music slot
    GalaxyArena,
}

impl Id {
    pub const VALUES: [Self; 43] = [
        Self::LuigiCircuit,
        Self::MooMooMeadow,
        Self::MushroomGorge,
        Self::ToadsFactory,
        Self::MarioCircuit,
        Self::CoconutMall,
        Self::DKSummit,
        Self::WariosGoldMine,
        Self::DaisyCircuit,
        Self::KoopaCape,
        Self::MapleTreeway,
        Self::GrumbleVolcano,
        Self::DryDryRuins,
        Self::MoonviewHighway,
        Self::BowsersCastle,
        Self::RainbowRoad,
        Self::PeachBeachGCN,
        Self::YoshiFallsDS,
        Self::GhostValleySNES,
        Self::MarioRacewayN64,
        Self::SherbetLandN64,
        Self::ShyGuyBeachGBA,
        Self::DelfinoSquareDS,
        Self::WaluigiStadiumGCN,
        Self::DesertHillsDS,
        Self::BowserCastleGBA,
        Self::JungleParkwayN64,
        Self::MarioCircuitGCN,
        Self::MarioCircuitSNES,
        Self::PeachGardenDS,
        Self::DKMountainGCN,
        Self::BowserCastleN64,
        Self::BlockPlaza,
        Self::DelfinoPier,
        Self::FunkyStadium,
        Self::ChompWheel,
        Self::ThwompDesert,
        Self::BattleCourseSNES,
        Self::BattleCourseGBA,
        Self::SkyscraperN64,
        Self::CookieLandGCN,
        Self::TwilightHouseDS,
        Self::GalaxyArena,
    ];

    pub fn as_str(&self) -> &'static str {
        match self {
            Id::LuigiCircuit => "Luigi Circuit",
            Id::MooMooMeadow => "Moo Moo Meadow",
            Id::MushroomGorge => "Mushroom Gorge",
            Id::ToadsFactory => "Toad's Factory",
            Id::MarioCircuit => "Mario Circuit",
            Id::CoconutMall => "Coconut Mall",
            Id::DKSummit => "DK Summit",
            Id::WariosGoldMine => "Wario's Gold Mine",
            Id::DaisyCircuit => "Daisy Circuit",
            Id::KoopaCape => "Koopa Cape",
            Id::MapleTreeway => "Maple Treeway",
            Id::GrumbleVolcano => "Grumble Volcano",
            Id::DryDryRuins => "Dry Dry Ruins",
            Id::MoonviewHighway => "Moonview Highway",
            Id::BowsersCastle => "Bowser's Castle",
            Id::RainbowRoad => "Rainbow Road",
            Id::PeachBeachGCN => "GCN Peach Beach",
            Id::YoshiFallsDS => "DS Yoshi Falls",
            Id::GhostValleySNES => "SNES Ghost Valley 2",
            Id::MarioRacewayN64 => "N64 Mario Raceway",
            Id::SherbetLandN64 => "N64 Sherbet Land",
            Id::ShyGuyBeachGBA => "GBA Shy Guy Beach",
            Id::DelfinoSquareDS => "DS Delfino Square",
            Id::WaluigiStadiumGCN => "GCN Waluigi Stadium",
            Id::DesertHillsDS => "DS Desert Hills",
            Id::BowserCastleGBA => "GBA Bowser Castle 3",
            Id::JungleParkwayN64 => "N64 DK Jungle Parkway",
            Id::MarioCircuitGCN => "GCN Mario Circuit",
            Id::MarioCircuitSNES => "SNES Mario Circuit 3",
            Id::PeachGardenDS => "DS Peach Garden",
            Id::DKMountainGCN => "GCN DK Mountains",
            Id::BowserCastleN64 => "N64 Bowser Castle",
            Id::BlockPlaza => "Block Plaza",
            Id::DelfinoPier => "Delfino Pier",
            Id::FunkyStadium => "Funky Stadium",
            Id::ChompWheel => "Chain Chomp Wheel",
            Id::ThwompDesert => "Thwomp Desert",
            Id::BattleCourseSNES => "SNES Battle Course 4",
            Id::BattleCourseGBA => "GBA Battle Course 3",
            Id::SkyscraperN64 => "N64 Skyscraper",
            Id::CookieLandGCN => "GCN Cookie Land",
            Id::TwilightHouseDS => "DS Twilight House",
            Id::GalaxyArena => "Galaxy Colosseum",
        }
    }
}
