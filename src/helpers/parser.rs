use std::{fs, io::Cursor, path::PathBuf, str, vec};

use crate::{
    apps::{
        tracks::{CupSettings, TrackDefinition},
        CheatCodeApp, EngineProbSet, SettingApp, SpeedometerMode, TrackDefApp,
    },
    Distro,
};

const MAGIC: &'static str = "ZRP0DIB1";
// Later should be moved to other source code
// Increment per push
const FILE_BUILD_NUMBER: &'static [u8; 2] = &[0, 4];

impl Distro {
    pub fn encode(&self) -> Vec<u8> {
        let mut m: Vec<u8> = vec![];
        let mut initial_size: u32 = 16;
        // Short for ZR Project 0 + Distribution
        let mut magic = MAGIC.as_bytes().to_vec();

        let mut setting = encode_settings(&self.settings);
        let mut cheats = encode_cheats(&self.codes);
        let mut cup = encode_cups(&self.tracks.editor);

        initial_size += (setting.len() + cup.len() + cheats.1.len()) as u32;

        m.append(&mut magic);
        initial_size.to_be_bytes().map(|v| m.push(v));
        FILE_BUILD_NUMBER.map(|v| m.push(v));
        m.append(&mut cheats.0);
        m.append(&mut setting);
        m.append(&mut cup);
        m.append(&mut cheats.1);

        m
    }

    pub fn decode(&mut self, path: &PathBuf) -> Result<Self, &'static str> {
        let file = fs::read(path).unwrap();
        if str::from_utf8(&file[0..8]).unwrap() != MAGIC && &file[12..14] == FILE_BUILD_NUMBER.as_slice() {
            return Err("The opened file is not a project file or failed on parsing for any reasons.");
        }

        let readable_size = as_u32_be(&file[8..12]) as usize;
        let mut loc_settings = &file[14..readable_size];
        let mut loc_cup = &file[48..];

        let cheat_enabled = loc_settings.take(..2).unwrap()[1] != 0;

        let cup_flag = loc_cup.take(..1).unwrap()[0];

        let distro = Self {
            path: Some(path.to_path_buf()),
            settings: decode_settings(loc_settings),
            codes: CheatCodeApp {
                enabled: cheat_enabled,
                ..Default::default()
            },
            tracks: TrackDefApp {
                editor: TrackDefinition {
                    mode: CupSettings {
                        nintendo: (0b001u8 & cup_flag) != 0,
                        nin_swap: (0b010u8 & cup_flag) != 0,
                        wiimm_cup: (0b100u8 & cup_flag) != 0,
                    },
                    ..Default::default()
                },
            },
            ..Default::default()
        };

        Ok(distro)
    }
}

pub fn encode_settings(s: &SettingApp) -> Vec<u8> {
    let mut pl: Vec<u8> = vec![];

    // 200cc
    pl.push(s.toggle_200cc as u8);
    // Engine probabilities
    for ele in s.engine_probs.low.to_be_bytes() {
        pl.push(ele);
    }
    for ele in s.engine_probs.mid.to_be_bytes() {
        pl.push(ele);
    }
    for ele in s.engine_probs.high.to_be_bytes() {
        pl.push(ele);
    }
    // CT TT
    pl.push(s.toggle_ct_tt as u8);
    // XPF
    pl.push(s.toggle_custom_presence as u8);
    // Block tracks
    pl.push(s.prevent_selection_online as u8);
    //SOM
    pl.push(s.toggle_som as u8);
    // Blue
    pl.push(s.toggle_drag_blue as u8);
    // Cloud
    for ele in s.time_cloud.to_be_bytes() {
        pl.push(ele);
    }

    let unfilled = 16 - (pl.len() % 16);
    pl.append(&mut zeros(unfilled));

    pl
}

pub fn encode_cups(c: &TrackDefinition) -> Vec<u8> {
    let mut pl: Vec<u8> = vec![];

    // Cup Settings
    pl.push((c.mode.nintendo as u8) + ((c.mode.nin_swap as u8) << 1) + ((c.mode.wiimm_cup as u8) << 2));
    pl.append(&mut zeros(1));
    // Cup Length
    pl.append(&mut (c.cups.len() as u16).to_be_bytes().to_vec());

    let unfilled = 16 - (pl.len() % 16);
    pl.append(&mut zeros(unfilled));

    // Cup Sets
    for cup in &c.cups {
        let mut cl: Vec<u8> = vec![];

        // Cup name
        let mut cup_name: Vec<u8> = (cup.name.len() as u8).to_be_bytes().to_vec();
        cup_name.append(&mut cup.name.clone().into_bytes());
        cup_name.append(&mut zeros(8 - (cup_name.len() % 8)));
        cl.append(&mut cup_name);

        // Icon binary
        let mut icon_bin: Vec<u8> = cup.icon.image.len().to_be_bytes().to_vec();
        icon_bin.append(&mut cup.icon.image.to_vec());
        icon_bin.append(&mut zeros(8 - (icon_bin.len() % 8)));
        cl.append(&mut icon_bin);
        // Icon filename
        let mut icon_filename: Vec<u8> = (cup.icon.filename.len() as u8).to_be_bytes().to_vec();
        icon_filename.append(&mut cup.icon.filename.clone().into_bytes());
        icon_filename.append(&mut zeros(8 - (icon_filename.len() % 8)));
        cl.append(&mut icon_filename);

        // Tracks
        for track in &cup.trackset {
            // Unused ID
            let mut id: Vec<u8> = track.id.to_be_bytes().to_vec();
            cl.append(&mut id);

            // Name
            let mut name: Vec<u8> = (track.name.len() as u8).to_be_bytes().to_vec();
            name.append(&mut track.name.clone().into_bytes());
            name.append(&mut zeros(8 - (name.len() % 8)));
            cl.append(&mut name);

            // Author
            let mut author: Vec<u8> = (track.author.len() as u8).to_be_bytes().to_vec();
            author.append(&mut track.author.clone().into_bytes());
            author.append(&mut zeros(8 - (author.len() % 8)));
            cl.append(&mut author);

            // Special IDs
            cl.push(track.property as u8);
            cl.push(track.music as u8);

            // New track flag
            cl.push(track.new as u8);

            // Group flag
            cl.push(track.flag as u8);

            // Filename
            let mut filename: Vec<u8> = (track.filename.len() as u8).to_be_bytes().to_vec();
            filename.append(&mut track.filename.clone().into_bytes());
            filename.append(&mut zeros(8 - (filename.len() % 8)));
            cl.append(&mut filename);
        }

        pl.append(&mut cl);
    }

    let unfilled = 16 - (pl.len() % 16);
    pl.append(&mut zeros(unfilled));

    pl
}

pub fn encode_cheats(c: &CheatCodeApp) -> (Vec<u8>, Vec<u8>) {
    let b = c.enabled() as u16;
    let mut pl: Vec<u8> = vec![];

    let codes = c.codes.as_ref();
    for code in codes {
        pl.append(&mut (code.name.len() as u16).to_be_bytes().to_vec());
        pl.append(&mut code.clone().name.into_bytes());
        pl.append(&mut zeros(8 - (pl.len() % 8)));

        let mut code_ntsc = code.clone().code_ntsc;
        code_ntsc.remove_matches(" ");
        code_ntsc.remove_matches("\n");
        pl.append(&mut (code_ntsc.len() as u32).to_be_bytes().to_vec());
        pl.append(&mut code_ntsc.into_bytes());
        pl.append(&mut zeros(8 - (pl.len() % 8)));

        let mut code_pal = code.clone().code_pal;
        code_pal.remove_matches(" ");
        code_pal.remove_matches("\n");
        pl.append(&mut (code_pal.len() as u32).to_be_bytes().to_vec());
        pl.append(&mut code_pal.into_bytes());
        pl.append(&mut zeros(8 - (pl.len() % 8)));

        let mut code_jp = code.clone().code_jp;
        code_jp.remove_matches(" ");
        code_jp.remove_matches("\n");
        pl.append(&mut (code_jp.len() as u32).to_be_bytes().to_vec());
        pl.append(&mut code_jp.into_bytes());
        pl.append(&mut zeros(8 - (pl.len() % 8)));

        let mut code_kor = code.clone().code_kor;
        code_kor.remove_matches(" ");
        code_kor.remove_matches("\n");
        pl.append(&mut (code_kor.len() as u32).to_be_bytes().to_vec());
        pl.append(&mut code_kor.into_bytes());
        pl.append(&mut zeros(8 - (pl.len() % 8)));
    }

    pl.append(&mut zeros(16 - (pl.len() % 16)));

    let len = (pl.len() as u32).to_be_bytes();

    for (l, v) in len.iter().enumerate() {
        pl[4 + l] = *v;
    }

    (b.to_be_bytes().to_vec(), pl)
}

fn decode_settings(a: &[u8]) -> SettingApp {
    SettingApp {
        toggle_200cc: a[0] != 0,
        engine_probs: EngineProbSet {
            low: as_u32_be(&a[1..5]) as i32,
            mid: as_u32_be(&a[5..9]) as i32,
            high: as_u32_be(&a[9..13]) as i32,
        },
        toggle_ct_tt: a[13] != 0,
        toggle_custom_presence: a[14] != 0,
        prevent_selection_online: a[15] as i8,
        toggle_som: SpeedometerMode::from_usize(a[16].into()).unwrap(),
        toggle_drag_blue: a[17] != 0,
        time_cloud: as_u16_be(&a[18..20]) as i16,
    }
}

pub fn decode_image(path: PathBuf) -> Result<Vec<u8>, image::ImageError> {
    let mut image: Vec<u8> = vec![];

    match image::open(path) {
        Ok(v) => {
            v.resize(128, 128, image::imageops::FilterType::Triangle)
                .write_to(&mut Cursor::new(&mut image), image::ImageOutputFormat::Png)
                .unwrap();
        }
        Err(err) => {
            return Err(err);
        }
    }

    Ok(image)
}

// Code below brings from Stack Overflow (https://stackoverflow.com/questions/29530011/creating-a-vector-of-zeros-for-a-specific-size),
// and edited to match our project
fn zeros(size: usize) -> Vec<u8> {
    let mut zero_vec: Vec<u8> = Vec::with_capacity(size);
    for _i in 0..size {
        zero_vec.push(0);
    }
    return zero_vec;
}

fn as_u32_be(array: &[u8]) -> u32 {
    ((array[0] as u32) << 24) + ((array[1] as u32) << 16) + ((array[2] as u32) << 8) + ((array[3] as u32) << 0)
}

fn as_u16_be(array: &[u8]) -> u16 {
    ((array[0] as u16) << 8) + ((array[1] as u16) << 0)
}
