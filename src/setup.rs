use std::{collections::HashMap, ffi::OsStr, fs, path};

fn get_template(path: &path::PathBuf) -> Option<(String, String)> {
    const FUNNY_MSG: &str = "unreachable in normal file systems";
    // rust could stabilize let expressions in bigger boolean expressions
    if path.extension() == Some(OsStr::new("html"))
        && path.file_name().expect(FUNNY_MSG) != OsStr::new("footer.html")
        && path.file_name().expect(FUNNY_MSG) != OsStr::new("header.html")
    {
        if let Ok(content) = fs::read_to_string(path) {
            let mut path = path.clone();
            path.set_extension("");
            Some((
                path.file_name()
                    .expect(FUNNY_MSG)
                    .to_str()
                    .expect(FUNNY_MSG)
                    .to_string(),
                content,
            ))
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
    pub global_style: Option<path::PathBuf>,
    pub global_script: Option<path::PathBuf>,
    pub templates: HashMap<String, String>,
}

impl Setup {
    pub fn load(dir: fs::ReadDir) -> Setup {
        Setup {
            global_style: {
                let path = path::PathBuf::from("_gabs").join("global.css");
                if path.exists() {
                    Some(path)
                } else {
                    None
                }
            },
            global_script: {
                let path = path::PathBuf::from("_gabs").join("global.css");
                if path.exists() {
                    Some(path)
                } else {
                    None
                }
            },
            templates: load_templates(dir),
        }
    }
}
