use chrono::Local;
use termion::color::{Fg, Rgb};
use termion::style::{Bold, Reset};

pub const GRAY: Fg<Rgb> = Fg(Rgb(153, 153, 153));
pub const ORANGE: Fg<Rgb> = Fg(Rgb(237, 76, 5));
pub const PURPLE: Fg<Rgb> = Fg(Rgb(141, 29, 117));
pub const GREEN: Fg<Rgb> = Fg(Rgb(0, 169, 51));
pub const RED: Fg<Rgb> = Fg(Rgb(255, 0, 0));
pub const YELLOW: Fg<Rgb> = Fg(Rgb(255, 255, 102));

pub fn info<S: Into<String>>(message: S) {
    let now = Local::now();
    let (year, month, day, hour, minutes, seconds) = (
        now.format("%Y").to_string(),
        now.format("%m").to_string(),
        now.format("%d").to_string(),
        now.format("%H").to_string(),
        now.format("%M").to_string(),
        now.format("%S").to_string(),
    );

    println!(
        "{}{}[{}{}{}-{}{}{}-{}{} {}{}{}:{}{}{}:{}{}{}] {}INFO{}: {}{}",
        Bold,
        GRAY,
        ORANGE,
        year,
        GRAY,
        ORANGE,
        month,
        GRAY,
        ORANGE,
        day,
        PURPLE,
        hour,
        GRAY,
        PURPLE,
        minutes,
        GRAY,
        PURPLE,
        seconds,
        GRAY,
        GREEN,
        GRAY,
        Reset,
        message.into()
    );
}

pub fn error<S: Into<String>>(message: S) {
    let now = Local::now();
    let (year, month, day, hour, minutes, seconds) = (
        now.format("%Y").to_string(),
        now.format("%m").to_string(),
        now.format("%d").to_string(),
        now.format("%H").to_string(),
        now.format("%M").to_string(),
        now.format("%S").to_string(),
    );

    println!(
        "{}{}[{}{}{}-{}{}{}-{}{} {}{}{}:{}{}{}:{}{}{}] {}ERROR{}: {}{}",
        Bold,
        GRAY,
        ORANGE,
        year,
        GRAY,
        ORANGE,
        month,
        GRAY,
        ORANGE,
        day,
        PURPLE,
        hour,
        GRAY,
        PURPLE,
        minutes,
        GRAY,
        PURPLE,
        seconds,
        GRAY,
        RED,
        GRAY,
        Reset,
        message.into()
    );
}