use crate::input_menu;
use std::{io, str::FromStr};

pub fn get<T: FromStr>(content: Option<&str>) -> Option<T> {
    input_menu::show(content.unwrap_or(""));
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    buffer.trim().parse::<T>().ok()
}
