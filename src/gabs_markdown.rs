use crate::setup::Setup;
use comrak::{markdown_to_html, ComrakOptions};

const HEADER_PREFIX: &str = "#!gabs";

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

    todo!();
    md
}
