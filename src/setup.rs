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
    pub has_global_style: bool,
    pub has_global_script: bool,
    pub templates: HashMap<String, String>,
}

impl Setup {
    pub fn load(dir: fs::ReadDir) -> Setup {
        Setup {
            has_global_style: path::PathBuf::from("_gabs").join("global.css").exists(),
            has_global_script: path::PathBuf::from("_gabs").join("global.js").exists(),
            templates: load_templates(dir),
        }
    }
}
