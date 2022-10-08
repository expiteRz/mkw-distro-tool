pub struct TrackDefinition {
    pub tracks: Vec<Track>,
}

pub struct Track {
    id: u16,
    name: String,
    property: Id,
    music: Id,
    flag: GroupFlag,
    filename: String,
}

pub enum GroupFlag {
    None,
    Header,
    Child,
}

impl Default for TrackDefinition {
    fn default() -> Self {
        Self {
            tracks: vec![
                Track::default(),
                Track::default(),
                Track::default(),
                Track::default(),
            ],
        }
    }
}

impl Default for Track {
    fn default() -> Self {
        Self {
            id: 0,
            name: "Null".to_string(),
            property: Id::MarioCircuit,
            music: Id::MarioCircuit,
            flag: GroupFlag::None,
            filename: "null.szs".to_string(),
        }
    }
}

impl Track {
    pub fn view(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            // Property
            egui::ComboBox::new("property", "")
                .selected_text(self.property.as_str())
                .show_ui(ui, |ui| {
                    for x in Id::VALUES {
                        ui.selectable_value(&mut &self.property, &x, format!("{}", x.as_str()));
                    }
                });
            // Music
            egui::ComboBox::new("music", "")
                .selected_text(self.music.as_str())
                .show_ui(ui, |ui| {
                    for x in 0..(Id::VALUES.len() - 1) {
                        ui.selectable_value(
                            &mut &self.music,
                            &Id::VALUES[x],
                            format!("{}", Id::VALUES[x].as_str()),
                        );
                    }
                })
        });
    }
}

#[derive(Debug, PartialEq)]
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

/// Test widget
pub fn test_view(ui: &mut egui::Ui) {
    let mut test_track_layout = |id: usize| {ui.horizontal(|ui| {
        ui.add(egui::TextEdit::singleline(&mut "Track name").desired_width(150.0).hint_text("Track name"));
        ui.separator();
        egui::ComboBox::new(format!("property_{}", id), "Property")
            .selected_text(format!("{}", Id::LuigiCircuit.as_str()))
            .width(130.0)
            .show_ui(ui, |ui| {
                for x in Id::VALUES {
                    ui.selectable_value(&mut &Id::LuigiCircuit, &x, format!("{}", x.as_str()));
                }}
            );
        ui.separator();
        egui::ComboBox::new(format!("music_{}", id), "Music")
            .selected_text(format!("{}", Id::LuigiCircuit.as_str()))
            .width(150.0)
            .show_ui(ui, |ui| {
                for x in 0..(Id::VALUES.len() - 1) {
                    ui.selectable_value(
                        &mut &Id::LuigiCircuit,
                        &Id::VALUES[x],
                        format!("{}", Id::VALUES[x].as_str()),
                    );
                }
            });
        ui.separator();
        ui.add(egui::TextEdit::singleline(&mut "path to track file").hint_text("Path to track file"));
        ui.small_button("...");
    });};
    for x in 0..4{
        test_track_layout(x);
    }
}
