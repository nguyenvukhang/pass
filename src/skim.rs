use std::io::Cursor;

use skim::prelude::*;

struct Name(String);

impl SkimItem for Name {
    fn text(&self) -> Cow<str> {
        Cow::Borrowed(&self.0)
    }
    fn preview(&self, _context: PreviewContext) -> ItemPreview {
        ItemPreview::Text(self.0.to_string())
    }
}

/// Default options for skim
fn options<'a>() -> Option<SkimOptions<'a>> {
    SkimOptionsBuilder::default()
        .height(Some("7"))
        .reverse(true)
        .color(Some("hl:-1"))
        .no_mouse(true)
        .build()
        .ok()
}

/// Use skim to select a choice in a list of strings
pub fn select_one(mut choices: Vec<String>) -> Option<String> {
    choices.sort();

    let input = choices.join("\n");
    let item_reader = SkimItemReader::default();
    let items = item_reader.of_bufread(Cursor::new(input));

    println!("----------------------");
    Skim::run_with(&options()?, Some(items))
        .and_then(|mut out| match out.final_key {
            Key::ESC | Key::Ctrl('c') => None,
            _ => out.selected_items.pop(),
        })
        .map(|v| v.output().to_string())
}
