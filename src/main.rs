use gabs::{build, init};
use std::{env, io};

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => build::build(),
        x => {
            if args[1] == "init" {
                init::init()
            } else {
                build::build()
            }
        }
    }
}
