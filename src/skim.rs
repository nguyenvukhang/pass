use crate::app::App;

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

fn skim(app: App) {
    let options = SkimOptionsBuilder::default()
        .height(Some("10"))
        .multi(false)
        .no_clear(false)
        .build()
        .unwrap();

    let db = app.read().unwrap();
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
