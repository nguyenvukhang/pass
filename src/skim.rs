use crate::database::Database;

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

/// Use skim to select a choice in a list of strings
pub fn select_one<I: Iterator<Item = String>>(choices: I) -> Option<String> {
    let opts = SkimOptionsBuilder::default().height(Some("10")).build().ok()?;
    let (tx, rx): (SkimItemSender, SkimItemReceiver) = unbounded();

    choices.for_each(|v| {
        let _ = tx.send(Arc::new(Name(v)));
    });

    drop(tx); // so that skim could know when to stop waiting for more items.

    Skim::run_with(&opts, Some(rx))
        .and_then(|mut out| out.selected_items.pop())
        .map(|v| v.output().to_string())
}

pub fn skim(db: Database) {
    let options =
        SkimOptionsBuilder::default().height(Some("10")).build().unwrap();

    let (tx_item, rx_item): (SkimItemSender, SkimItemReceiver) =
        skim::prelude::bounded(db.count());
    for name in db.list_all() {
        let _ = tx_item.send(Arc::new(Name(name)));
    }
    drop(tx_item); // so that skim could know when to stop waiting for more items.

    // `run_with` would read and show items from the stream
    let selected = Skim::run_with(&options, Some(rx_item))
        .and_then(|mut out| out.selected_items.pop());

    let selected = match selected {
        Some(v) => v,
        None => return,
    };

    let name = selected.output();
    let password = db.get_unchecked(&name);

    print!("{} -> {}\n", name, password);
}
