use crate::{gabs_markdown, init, setup::Setup};
use std::{env, ffi, fs, io, path};

#[macro_export]
macro_rules! format_pathbuf {
    ($path:expr) => {
        ($path).as_os_str().to_str().unwrap_or_else(|| "<unknown>")
    };
}

fn builded_path(path: &path::PathBuf) -> path::PathBuf {
    path.strip_prefix("_gabs")
        .expect("unreachable")
        .to_path_buf()
}

fn build_md(path: &path::PathBuf, setup: &Setup) {
    if let Ok(md) = fs::read_to_string(path) {
        let mut bpath = builded_path(path).clone();
        bpath.set_extension("html");
        if let Err(_) = fs::write(bpath.clone(), &gabs_markdown::md2html(md, setup)[..]) {
            eprintln!(
                "ERROR: Could not write to {}, skipping.",
                format_pathbuf!(bpath)
            );
        }
    } else {
        eprintln!(
            "ERROR: Could not read from file {}, skipping.",
            format_pathbuf!(path)
        )
    }
}

macro_rules! copy_to_build {
    ($file:ident) => {{
        let bpath = builded_path($file);
        if let Err(_) = fs::copy($file.clone(), bpath) {
            eprintln!(
                "Copying from {} to {} failed. A corrupted file can be at the destination path.",
                format_pathbuf!($file),
                format_pathbuf!($file)
            );
        };
    }};
}

fn build_file(path: &path::PathBuf, setup: &Setup) {
    // boilerplate lol, is the borrow checker really necessary?
    let markdown = ffi::OsString::from("markdown");
    let markdown = markdown.as_os_str();
    let md = ffi::OsString::from("md");
    let md = md.as_os_str();
    let html = ffi::OsString::from("html");
    let html = html.as_os_str();
    if let Some(ext) = path.extension() {
        if ext == markdown || ext == md {
            build_md(path, setup);
        } else if ext == html {
            return;
        } else {
            copy_to_build!(path);
        }
    } else {
        copy_to_build!(path);
    }
}

fn build_dir(dir: fs::ReadDir, setup: &Setup) {
    for entry in dir {
        if let Ok(entry) = entry {
            if let Ok(filetype) = entry.file_type() {
                if filetype.is_file() {
                    build_file(&entry.path(), setup);
                } else if filetype.is_dir() {
                    if let Ok(dir) = fs::read_dir(&entry.path()) {
                        fs::create_dir(entry.path().strip_prefix("_gabs").expect("unrechable"))
                            .ok();
                        build_dir(dir, setup);
                    } else {
                        eprintln!(
                            "ERROR: Could not enter directory {}, skipping.",
                            format_pathbuf!(entry.path())
                        );
                    }
                } else if filetype.is_symlink() {
                    eprintln!("ERROR: Current version doesn't follows symlinks. SORRY! I'LL PATCH THIS SOON! Skipping {}", format_pathbuf!(entry.path()));
                }
            } else {
                eprintln!(
                    "ERROR: Could not get entry type for {}, skipping.",
                    format_pathbuf!(entry.path())
                );
            }
        } else {
            panic!("Intermitent IO error during build! It's possible that remainings of the build (possibly corrupted) are still inside in the directory");
        }
    }
}

macro_rules! set_current_dir_or_panic {
    ($dir:ident) => {{
        if env::set_current_dir($dir).is_err() {
            panic!("Could not set the current directory.");
        }
    }};
}

fn goto_project_directory(path: &path::Path) -> Result<(), io::Error> {
    if let Ok(dir) = fs::read_dir("_gabs") {
        build_dir(dir, &Setup::load(fs::read_dir("_gabs").unwrap()));
        Ok(())
    } else {
        if let Some(parent) = path.parent() {
            set_current_dir_or_panic!(parent);
            goto_project_directory(&parent)
        } else {
            Err(io::Error::new(io::ErrorKind::NotFound, ""))
        }
    }
}

pub fn build() -> Result<(), io::Error> {
    let cwd = env::current_dir().expect("Could not get current directory.");
    let cwd = cwd.as_path();
    let ret = goto_project_directory(&cwd);
    if ret.is_err() {
        set_current_dir_or_panic!(cwd);
        init::init()?;
        build()
    } else {
        ret
    }
}
