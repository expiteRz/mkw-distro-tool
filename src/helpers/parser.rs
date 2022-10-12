use std::{
    fs::{self},
    path::PathBuf,
    str, vec,
};

use crate::{
    apps::{CheatCodeApp, EngineProbSet, SettingApp, SpeedometerMode},
    Distro,
};

const MAGIC: &'static str = "ZRP0DIB1";
const FILE_BUILD_NUMBER: &'static [u8; 2] = &[0, 1]; // Later should be moved to other source code

impl Distro {
    pub fn encode(&self) -> Vec<u8> {
        let mut m: Vec<u8> = vec![];
        let mut initial_size: u32 = 16;
        // Short for ZR Project 0 + Distribution
        let mut magic = MAGIC.as_bytes().to_vec();

        let mut setting = encode_settings(&self.settings);
        let (mut cheat_enabled, _) = encode_cheats(&self.codes);

        initial_size += setting.len() as u32;

        m.append(&mut magic);
        m.append(&mut initial_size.to_be_bytes().to_vec());
        FILE_BUILD_NUMBER.map(|v| {
            m.push(v);
        });
        m.append(&mut cheat_enabled);
        m.append(&mut setting);

        m
    }

    pub fn decode(&mut self, path: &PathBuf) -> Result<Self, ()> {
        let file = fs::read(path).unwrap();
        if str::from_utf8(&file[0..8]).unwrap() != MAGIC && &file[12..14] == FILE_BUILD_NUMBER.as_slice() {
            return Err(());
        }

        let readable_size = as_u32_be(&file[8..12]) as usize;
        let mut loc = &file[14..readable_size];

        let cheat_enabled = loc.take(..2).unwrap()[1] != 0;

        let distro = Self {
            path: Some(path.to_path_buf()),
            settings: decode_settings(loc),
            codes: CheatCodeApp {
                enabled: cheat_enabled,
                ..Default::default()
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

pub fn encode_cheats(c: &CheatCodeApp) -> (Vec<u8>, Vec<u8>) {
    let b = c.enabled() as u16;

    (b.to_be_bytes().to_vec(), vec![])
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
