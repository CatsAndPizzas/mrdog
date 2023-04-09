use std::fmt::Display;

use super::change_request::ChangeRequest;
struct Field {
    text: String,
    pub printable_length: usize,
}

impl Field {
    fn plain(input: &str) -> Self {
        Field {
            text: input.to_string(),
            printable_length: input.len(),
        }
    }
    fn hyperlink(text: &str, url: &str) -> Self {
        // See https://gist.github.com/egmontkob/eb114294efbcd5adb1944c9f3cb5feda on hyperlink syntax
        // See https://stackoverflow.com/a/33139393/4425335 on escape sequences in Rust
        let display_text = format!("\x1b]8;;{url}\x07{text}\x1b]8;;\x07");
        Field {
            text: display_text,
            printable_length: text.chars().count(),
        }
    }
    fn concat(fields: &[Field]) -> Self {
        fields.iter().fold(
            Field {
                text: "".to_string(),
                printable_length: 0,
            },
            |result, field| {
                if result.text.is_empty() {
                    Field {
                        text: field.text.clone(),
                        printable_length: field.printable_length,
                    }
                } else {
                    Field {
                        text: result.text + &format!(", {}", field.text),
                        printable_length: result.printable_length + field.printable_length + 2,
                    }
                }
            },
        )
    }
}

impl Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text)
    }
}

pub fn print_change_requests(crs: &[ChangeRequest]) {
    let mut rows = Vec::new();

    rows.push(vec![
        Field::plain("Project"),
        Field::plain("Id"),
        Field::plain("Assignees"),
        Field::plain("Reviewers"),
        Field::plain("Title"),
    ]);

    for c in crs {
        rows.push(vec![
            Field::hyperlink(&c.project.display_name, &c.project.url),
            Field::hyperlink(&c.id, &c.url),
            Field::concat(
                &c.assignees
                    .iter()
                    .map(|a| Field::hyperlink(&a.display_name, &a.url))
                    .collect::<Vec<_>>(),
            ),
            Field::concat(
                &c.reviewers
                    .iter()
                    .map(|a| Field::hyperlink(&a.display_name, &a.url))
                    .collect::<Vec<_>>(),
            ),
            Field::hyperlink(&c.title, &c.url),
        ])
    }

    for (r_i, row) in rows.iter().enumerate() {
        for (c_i, col) in row.iter().enumerate() {
            let width = rows.iter().map(|r| &r[c_i].printable_length).max().unwrap();
            let pad = width - col.printable_length;
            print!("{col}{} ", " ".repeat(pad))
        }
        println!();
        if r_i == 0 {
            for (c_i, _) in row.iter().enumerate() {
                let width = rows.iter().map(|r| &r[c_i].printable_length).max().unwrap();
                print!("{} ", "-".repeat(*width))
            }
            println!()
        }
    }
}
