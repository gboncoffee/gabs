use crate::setup::Setup;
use std::{fs, io};

fn build_dir(dir: fs::ReadDir, setup: Setup) -> Result<(), io::Error> {
    todo!();
    Ok(())
}

pub fn build() -> Result<(), io::Error> {
    if let Ok(dir) = fs::read_dir("_gabs") {
        build_dir(dir, Setup::load(fs::read_dir("_gabs").unwrap()))
    } else {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Could not read directory _gabs. Are you inside a Gabs website?",
        ))
    }
}
