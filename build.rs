#![allow(unused_imports, dead_code, unused_variables)]

use std::io::{Write, BufRead};

fn main() -> std::io::Result<()> {
    const FILE_PATH: &'static str = "./res/tracks.bin";
    let _ = std::fs::remove_file(FILE_PATH);

    const MAGIC: &'static str = "ZRP0TRC0";
    const BUILD_NUMBER: &'static u8 = &1;
    const ERROR_COMPRESS: &'static str = "Failed to compress file";

    let mut track_bin: Vec<u8> = vec![];
    let track_path = match std::fs::read_dir("tracks") {
        Ok(dir) => dir,
        Err(err) => {
            println!("Not a directory");
            return Err(err);
        }
    };

    track_bin.append(&mut MAGIC.as_bytes().to_vec());
    track_bin.append(&mut zeros(8));

    for v in track_path {
        match v {
            Ok(v) => {
                if v.file_type().unwrap().is_file() && v.path().extension().is_some() && (v.path().extension().unwrap().to_str().unwrap()) == "szs" {
                    let name = format!("{}", base64::encode(v.file_name().to_str().unwrap().as_bytes()));
                    let name_len = name.len() as u16;
                    let data = std::fs::read(v.path()).unwrap();
                    let mut encoded = zstd::bulk::compress(&data, 0).expect(ERROR_COMPRESS);
                    let length = encoded.len() as u32;

                    track_bin.append(&mut name_len.to_be_bytes().to_vec());
                    track_bin.append(&mut name.as_bytes().to_vec());
                    track_bin.append(&mut zeros(16 - (track_bin.len() % 16)));
                    track_bin.append(&mut length.to_be_bytes().to_vec());
                    track_bin.append(&mut encoded);
                    track_bin.append(&mut zeros(16 - (track_bin.len() % 16)));
                };
            }
            Err(err) => {
                println!("{}", ERROR_COMPRESS);
                return Err(err);
            }
        }
    }

    let track_bin_len = (track_bin.len() as u32).to_be_bytes();
    track_bin[8] = track_bin_len[0];
    track_bin[9] = track_bin_len[1];
    track_bin[10] = track_bin_len[2];
    track_bin[11] = track_bin_len[3];
    track_bin[15] = *BUILD_NUMBER;

    let mut file = std::fs::File::create(FILE_PATH).expect("Failed to create a binary file");
    file.write_all(&track_bin)
}

fn zeros(size: usize) -> Vec<u8> {
    let mut zero_vec: Vec<u8> = Vec::with_capacity(size);
    for _i in 0..size {
        zero_vec.push(0);
    }
    zero_vec
}
