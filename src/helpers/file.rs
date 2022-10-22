use std::{fs::File, io::Write, path::PathBuf};

use crate::{
    apps::tracks::{Cup, Track},
    Distro,
};

use super::parser::decode_image;

const IMAGE_FORMAT: &'static [&'static str] = &["png", "jpg", "jpeg", "gif", "bmp"];

impl Distro {
    pub fn save_project(&self, path: &PathBuf) -> Result<(), std::io::Error> {
        let mut f = File::create(path).unwrap();
        let b = self.encode();
        f.write_all(&b[..])
    }

    pub fn open_project(&mut self, path: &PathBuf) {
        match self.decode(path) {
            Ok(v) => {
                self.path = v.path;
                self.codes = v.codes;
                self.settings = v.settings;
                self.tracks = v.tracks;
            }
            Err(err) => {
                self.err_msg = err;
                self.confirm_dialog = true;
            }
        };
    }
}

impl Cup {
    pub fn open_image(&mut self) {
        match rfd::FileDialog::new()
            .add_filter("Image file", IMAGE_FORMAT)
            .pick_file()
        {
            Some(path) => {
                match decode_image(path.clone()) {
                    Ok(v) => {
                        self.icon.filename = format!("{}", path.file_name().unwrap().to_str().unwrap());
                        self.icon.image = v;
                    }
                    Err(_) => {}
                };
            }
            None => {}
        }
    }
}

impl Track {
    pub fn open_file(&mut self) -> Result<(), &'static str> {
        match rfd::FileDialog::new()
            .add_filter("*.szs", &["szs"])
            .pick_file()
        {
            Some(path) => self.filename = format!("{}", path.to_str().unwrap()),
            None => return Err("Failed to get szs file"),
        }
        Ok(())
    }
}
