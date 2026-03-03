use std::io::{self, Write};

pub fn show(content: &str) {
    print!("{content} > ");
    io::stdout().flush().unwrap();
}
