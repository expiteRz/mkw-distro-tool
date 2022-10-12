use std::{fs::File, io::Write, path::PathBuf};

use crate::Distro;

impl Distro {
    pub fn save_file(&self, path: &PathBuf) -> Result<(), std::io::Error> {
        let mut f = File::create(path).unwrap();
        let b = self.encode();
        f.write_all(&b[..])
    }

    pub fn open_file(&mut self, path: &PathBuf) {
        let d = self.decode(path).unwrap();

        self.path = d.path;
        self.codes = d.codes;
        self.settings = d.settings;
    }
}
