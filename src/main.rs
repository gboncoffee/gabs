use gabs::{build, init};
use std::io;

fn main() -> Result<(), io::Error> {
    if let Err(bres) = build::build() {
        if bres.kind() == io::ErrorKind::NotFound {
            init::init()?;
            build::build()
        } else {
            Ok(())
        }
    } else {
        Ok(())
    }
}
