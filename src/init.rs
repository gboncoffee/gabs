use std::{fs, io, path};

macro_rules! write_default {
    ($name:literal) => {
        if let Err(_) = fs::write(
            path::PathBuf::from("_gabs").join($name),
            include_str!($name),
        ) {
            Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                "Couldn't write to _gabs{path::MAIN_SEPARATOR}{$name}",
            ))
        } else {
            Ok(())
        }
    };
}

pub fn init() -> Result<(), io::Error> {
    if let Err(_) = fs::create_dir("_gabs") {
        Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            "Could not create _gabs directory",
        ))
    } else {
        write_default!("header.html")?;
        write_default!("footer.html")?;
        write_default!("index.md")?;
        Ok(())
    }
}
