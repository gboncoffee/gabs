use std::{collections::HashMap, ffi::OsStr, fs, path};

fn get_template(path: &path::PathBuf) -> Option<(String, String)> {
    // rust could stabilize let expressions in bigger boolean expressions
    if path.extension() == Some(OsStr::new("html")) {
        if let Ok(content) = fs::read_to_string(path) {
            Some((path.clone().set_extension("").to_string(), content))
        } else {
            None
        }
    } else {
        None
    }
}

fn load_templates(dir: fs::ReadDir) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for entry in dir {
        if let Ok(entry) = entry {
            if let Some((name, content)) = get_template(&path::PathBuf::from(entry.path())) {
                map.insert(name, content);
            }
        } else {
            panic!("Intermitent IO error during directory reading.");
        }
    }
    map
}

pub struct Setup {
    global_style: Option<path::PathBuf>,
    global_script: Option<path::PathBuf>,
    templates: HashMap<String, String>,
}

impl Setup {
    pub fn load(dir: fs::ReadDir) -> Setup {
        Setup {
            global_style: if let Ok(path) =
                fs::read_to_string(path::PathBuf::from("_gabs").join("global.css"))
            {
                Some(path::PathBuf::from(path))
            } else {
                None
            },
            global_script: if let Ok(path) =
                fs::read_to_string(path::PathBuf::from("_gabs").join("global.js"))
            {
                Some(path::PathBuf::from(path))
            } else {
                None
            },
            templates: load_templates(dir),
        }
    }
}
