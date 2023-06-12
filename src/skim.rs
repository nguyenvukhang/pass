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
        .no_mouse(true)
        .build()
        .ok()
}

/// Use skim to select a choice in a list of strings
pub fn select_one(mut choices: Vec<String>) -> Option<String> {
    let opts = options()?;
    let (tx, rx): (SkimItemSender, SkimItemReceiver) = unbounded();

    choices.sort();

    choices.into_iter().for_each(|v| {
        let _ = tx.send(Arc::new(Name(v)));
    });

    drop(tx); // so that skim could know when to stop waiting for more items.

    Skim::run_with(&opts, Some(rx))
        .and_then(|mut out| match out.final_key {
            Key::ESC | Key::Ctrl('c') => None,
            _ => out.selected_items.pop(),
        })
        .map(|v| v.output().to_string())
}
