extern crate tui;
extern crate termion;
extern crate chrono;
extern crate tempfile;

use std::io;

use termion::event;
use termion::input::TermRead;
use termion::screen::AlternateScreen;

use tui::Terminal;
use tui::backend::TermionBackend;
use tui::layout::{Group, Size};
use tui::widgets::Widget;

use chrono::offset::local::Local;

use self::calendar::Calendar;

mod editor;
mod month;
mod calendar;
mod week;
mod database;

fn one_day() -> chrono::Duration {
    chrono::Duration::days(1)
}

fn day_suffix(n: u32) -> &'static str {
    match (n % 10, (n % 100) / 10) {
        (1, x) if x != 1 => "st",
        (2, x) if x != 1 => "nd",
        (3, x) if x != 1 => "rd",
        _ => "th",
    }
}

const DB_LOC: &'static str = "/home/shum/org/gtg/cal";

const DAY_NAMES: [&'static str; 7] = [
    "Mon",
    "Tue",
    "Wed",
    "Thu",
    "Fri",
    "Sat",
    "Sun",
];

const MONTH_NAMES: [&'static str; 12] = [
    "Jan",
    "Feb",
    "Mar",
    "Apr",
    "May",
    "Jun",
    "Jul",
    "Aug",
    "Sep",
    "Oct",
    "Nov",
    "Dec",
];

fn main() {
    match run() {
        Ok(()) => (),
        Err(e) => {
            println!("Error! {}", e);
        },
    }
}

fn run() -> Result<(), io::Error> {
    let _alt_screen = AlternateScreen::from(io::stdout());

    let backend = TermionBackend::new()?;
    let mut terminal = Terminal::new(backend)?;

    terminal.clear()?;
    terminal.hide_cursor()?;

    let mut calendar = Calendar::new(Local::now().naive_local());

    let mut size = terminal.size()?;
    let mut keys = io::stdin().keys();
    loop {
        Group::default()
            .sizes(&[Size::Percent(100)])
            .render(&mut terminal, &size, |t, chunks| {
                calendar.render(t, &chunks[0])
            });
        terminal.draw()?;

        let c = match keys.next() {
            Some(c) => c?,
            None => break,
        };

        let new_size = terminal.size()?;
        if new_size != size {
            size = new_size;
            terminal.resize(size)?;
        }

        let redraw = match c {
            event::Key::Char('q') => break,
            event::Key::Left | event::Key::Char('h') => calendar.left(),
            event::Key::Down | event::Key::Char('j') => calendar.down(),
            event::Key::Up | event::Key::Char('k') => calendar.up(),
            event::Key::Right | event::Key::Char('l') => calendar.right(),
            event::Key::Char('\n') => calendar.enter(),
            event::Key::Char('>') => calendar.next_view(),
            event::Key::Char('<') => calendar.prev_view(),
            _ => false,
        };
        if redraw {
            terminal.hide_cursor()?;
            terminal.resize(size)?;
        }
    }

    terminal.show_cursor()?;
    Ok(())
}

