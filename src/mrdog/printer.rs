use super::change_request::ChangeRequest;
use prettytable::{color, Attr, Cell, Row};

pub fn print_change_requests(crs: &[ChangeRequest]) {
    let mut table = table!();

    for c in crs {
        table.add_row(Row::new(vec![Cell::new(&c.title), Cell::new(&c.url)]));
    }

    table.add_row(Row::new(vec![
        Cell::new("foobar")
            .with_style(Attr::Bold)
            .with_style(Attr::ForegroundColor(color::GREEN)),
        Cell::new("bar")
            .with_style(Attr::BackgroundColor(color::RED))
            .with_style(Attr::Italic(true))
            .with_hspan(2),
        Cell::new("foo"),
    ]));
    table.printstd()
}
