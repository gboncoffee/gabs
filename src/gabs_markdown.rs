use crate::setup::Setup;
use comrak;
use std::{fs, path};

const HEADER_PREFIX: &str = "#!gabs";

fn add_styles(html: &mut String, template: Option<&str>, setup: &Setup) {
    // add the global style before so the specific can override stuff
    if setup.has_global_style {
        html.push_str("<link rel=\"stylesheet\" href=\"/global.css\">");
    }

    let string = if let Some(template) = template {
        let mut stylesheet = path::PathBuf::from("_gabs").join(template);
        stylesheet.set_extension("css");
        if stylesheet.exists() {
            format!("<link rel=\"stylesheet\" href=\"/{template}.css\">\n")
        } else {
            String::from("")
        }
    } else {
        String::from("")
    };
    html.push_str(&string[..]);
}

fn add_scripts(html: &mut String, template: Option<&str>, setup: &Setup) {
    if setup.has_global_script {
        html.push_str("<script type=\"text/javascript\" src=\"/global.js\" defer></script>");
    }

    let string = if let Some(template) = template {
        let mut script = path::PathBuf::from("_gabs").join(template);
        script.set_extension("js");
        if script.exists() {
            format!("<script type=\"text/javascript\" src=\"/{template}.js\" defer></script>")
        } else {
            String::from("")
        }
    } else {
        String::from("")
    };
    html.push_str(&string[..]);
}

fn parse_template<'a>(template: Option<&'a str>, setup: &'a Setup) -> (&'a str, &'a str) {
    if let Some(template) = template {
        if let Some(html_t) = setup.templates.get(template) {
            let mut it = html_t.split("<!-- gabs -->");
            (
                it.next().expect("unreachable"),
                if let Some(s) = it.next() { s } else { "" },
            )
        } else {
            eprintln!("ERROR: No HTML template for {template}. Building without one...");
            ("", "")
        }
    } else {
        ("", "")
    }
}

fn try_add_file_on_gabs_dir(html: &mut String, file: &str) {
    if let Ok(file) = fs::read_to_string(path::PathBuf::from("_gabs").join(file)) {
        html.push_str(&file[..]);
    }
}

fn buildmd(md: String, template: Option<&str>, title: Option<&str>, setup: &Setup) -> String {
    let mut html = String::from(
        "<!DOCTYPE html>
<html>
<head>
<meta charset=\"UTF-8\">
",
    );
    // (maybe) add title
    if let Some(title) = title {
        html.push_str(&format!("<title>{title}</title>\n"));
    }
    add_styles(&mut html, template, setup);

    // init the body
    html.push_str(
        "</head>
<body>",
    );

    try_add_file_on_gabs_dir(&mut html, "header.html");

    add_scripts(&mut html, template, setup);

    // add the document itself
    let (first_half, second_half) = parse_template(template, setup);
    html.push_str(first_half);
    // gfm
    let mut options = comrak::ComrakOptions::default();
    options.extension.table = true;
    options.extension.tasklist = true;
    options.extension.strikethrough = true;
    options.extension.autolink = true;
    options.extension.tagfilter = true;
    html.push_str(&comrak::markdown_to_html(&md[..], &options)[..]);
    html.push_str(second_half);

    try_add_file_on_gabs_dir(&mut html, "footer.html");

    html
}

pub fn md2html(md: String, setup: &Setup) -> String {
    let mut lines: Vec<&str> = md.lines().collect();
    let (template, title) = if lines[0].starts_with(HEADER_PREFIX) {
        let mut split = lines[0]
            .strip_prefix(HEADER_PREFIX)
            .expect("unreachable")
            .split(':');
        let t = split.next().expect("unreachable").trim();
        lines.remove(0);
        (if t.len() > 0 { Some(t) } else { None }, split.next())
    } else {
        (None, None)
    };

    let mut md = String::new();
    for line in lines {
        md.push_str(line);
        md.push('\n');
    }

    buildmd(md, template, title, setup)
}
