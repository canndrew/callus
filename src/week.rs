use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::widgets::{border, Widget, Block};
use tui::style::{Color, Modifier, Style};

use chrono::naive::date::NaiveDate;
use chrono::offset::local::Local;
use chrono::{Datelike, Timelike};

use std::cmp::{min, max};
use std::sync::Arc;
use std::cell::RefCell;

use ::day_suffix;
use ::{DAY_NAMES, MONTH_NAMES, one_day};
use ::database::Database;

pub struct WeekView {
    selected_date: NaiveDate,
    selected_hour: u8,
    top_left_date: RefCell<NaiveDate>,
    top_hour: RefCell<u8>,
    db: Arc<Database>,
}

impl WeekView {
    pub fn new(db: Arc<Database>, date: NaiveDate, hour: u8) -> WeekView {
        let top_date = if hour == 0 {
            date - one_day()
        } else {
            date
        };
        let days_offset = top_date.weekday().num_days_from_monday();
        let days_offset = max(days_offset, 1);
        let days_offset = min(days_offset, 5);

        WeekView {
            selected_date: date,
            selected_hour: hour,
            top_left_date: RefCell::new(date - (one_day() * days_offset as i32)),
            top_hour: RefCell::new((hour + 23) % 24),
            db: db,
        }
    }

    pub fn set_date(&mut self, date: NaiveDate) {
        //let hour = Local::now().naive_local().hour() as u8;
        let hour = 11;
        let top_date = if hour == 0 {
            date - one_day()
        } else {
            date
        };
        let days_offset = top_date.weekday().num_days_from_monday();
        let days_offset = max(days_offset, 1);
        let days_offset = min(days_offset, 5);

        if date != self.selected_date {
            self.selected_date = date;
            self.selected_hour = hour;
            *self.top_left_date.get_mut() = date - (one_day() * days_offset as i32);
            *self.top_hour.get_mut() = (hour + 23) % 24;
        }
    }

    pub fn get_date(&self) -> NaiveDate {
        self.selected_date
    }

    pub fn up(&mut self) -> bool {
        if self.selected_hour == 0 {
            self.selected_hour = 23;
            self.selected_date = self.selected_date - one_day();
        } else {
            self.selected_hour -= 1;
        }
        false
    }

    pub fn down(&mut self) -> bool {
        if self.selected_hour == 23 {
            self.selected_hour = 0;
            self.selected_date = self.selected_date + one_day();
        } else {
            self.selected_hour += 1;
        }
        false
    }

    pub fn left(&mut self) -> bool {
        self.selected_date = self.selected_date - one_day();
        false
    }

    pub fn right(&mut self) -> bool {
        self.selected_date = self.selected_date + one_day();
        false
    }

    pub fn enter(&mut self) -> bool {
        let date = self.selected_date;
        let hour = self.selected_hour;
        self.db.edit_hour(date, hour);
        true
    }

    pub fn draw(&self, area: &Rect, buffer: &mut Buffer) {
        //let title = format!("{} {}", MONTH_NAMES[self.selected_date.month0() as usize], self.selected_date.year());
        //buffer.set_string((area.width - title.len() as u16) / 2, 0, &title, &Style::default().fg(Color::Yellow).modifier(Modifier::Bold));
        
        #[derive(Debug)]
        struct Row {
            show_date: bool,
            y: u16,
            hour: u8,
            box_height: u16,
            entries: Vec<Entry>,
        }

        #[derive(Debug)]
        struct Entry {
            summary: String,
            selected: bool,
            today: bool,
            date: NaiveDate,
        }

        let mut rows = Vec::new();
        let mut y;
        let mut left_date;
        let mut hour;
        let today = Local::now().naive_local().date();
        let now = Local::now().naive_local().hour() as u8;
        let mut redraw_count = 0;
        let mut move_left = false;
        let mut move_right = false;
        let mut move_up = false;
        let mut move_down = false;
        //let mut debug = Vec::new();
        'redraw: loop {
            if redraw_count == 30 {
                break;
            }
            redraw_count += 1;

            if move_left {
                let mut top_left_date = self.top_left_date.borrow_mut();
                *top_left_date = *top_left_date - one_day();
            }
            if move_right {
                let mut top_left_date = self.top_left_date.borrow_mut();
                *top_left_date = *top_left_date + one_day();
            }
            if move_up {
                let mut top_left_date = self.top_left_date.borrow_mut();
                let mut top_hour = self.top_hour.borrow_mut();
                if *top_hour == 0 {
                    *top_hour = 23;
                    *top_left_date = *top_left_date - one_day();
                } else {
                    *top_hour -= 1;
                }
            }
            if move_down {
                let mut top_left_date = self.top_left_date.borrow_mut();
                let mut top_hour = self.top_hour.borrow_mut();
                if *top_hour == 23 {
                    *top_hour = 0;
                    *top_left_date = *top_left_date + one_day();
                } else {
                    *top_hour += 1;
                }
            }

            rows = Vec::new();
            y = 0;
            left_date = *self.top_left_date.borrow();
            hour = *self.top_hour.borrow();
            let mut found_selected = false;
            loop {
                let mut new_row = Row {
                    show_date: false,
                    hour: hour,
                    y: y,
                    box_height: 3,
                    entries: Vec::new(),
                };
                if hour == 0 || y == 0 {
                    new_row.show_date = true;
                    y += 1;
                }
                if y > area.height {
                    break;
                }
                let max_height = area.height - y;
                let mut saw_selected = false;
                for day_offset in 0..7 {
                    let day = left_date + (one_day() * day_offset);
                    let selected = day == self.selected_date && hour == self.selected_hour && !found_selected;
                    let is_today = day == today && hour == now;
                    let summary = self.db.get_hour(day, hour);
                    new_row.box_height = max(new_row.box_height, summary.lines().count() as u16);
                    let new_entry = Entry {
                        summary: summary.to_owned(),
                        selected: selected,
                        today: is_today,
                        date: day,
                    };
                    new_row.entries.push(new_entry);

                    if selected && day_offset == 0 {
                        move_left = true;
                        continue 'redraw;
                    }
                    if selected && day_offset == 6 {
                        move_right = true;
                        continue 'redraw;
                    }
                    saw_selected = saw_selected || selected;
                    found_selected = found_selected || selected;
                }
                if saw_selected && y <= 1 {
                    move_up = true;
                    continue 'redraw;
                }
                if saw_selected && (y + new_row.box_height) >= area.height - 2 {
                    move_down = true;
                    continue 'redraw;
                }
                new_row.box_height = min(new_row.box_height, max_height);
                y += new_row.box_height;
                if y > area.height {
                    break;
                }
                hour = hour + 1;
                if hour == 24 {
                    hour = 0;
                    left_date = left_date + one_day();
                }
                rows.push(new_row);
            }
            break;
        }

        //println!("#rpws == {}", rows.len());
        for row in rows {
            for (day_offset, entry) in row.entries.into_iter().enumerate() {
                if row.show_date {
                    let day_of_month = entry.date.day();
                    let day_of_week = entry.date.weekday().num_days_from_monday();
                    let month = entry.date.month0();
                    let year = entry.date.year();
                    let column_title = format!("{} {}{} {} {}",
                            DAY_NAMES[day_of_week as usize],
                            day_of_month,
                            day_suffix(day_of_month),
                            MONTH_NAMES[month as usize],
                            year);
                    let x = 1 + (1 + day_offset as u16 * 2) * area.width / 14 - column_title.len() as u16 / 2;
                    buffer.set_string(x, row.y, &column_title, &Style::default().modifier(Modifier::Bold));
                }

                let this_x = (area.width + 1) * day_offset as u16 / 7;
                let next_x = (area.width + 1) * (day_offset as u16 + 1) / 7;
                let rect = Rect {
                    x: this_x,
                    y: row.y + if row.show_date { 1 } else { 0 },
                    width: next_x - this_x - if day_offset == 6 { 1 } else { 0 },
                    height: row.box_height,
                };

                let fg = match entry.today {
                    true => Some(Color::Yellow),
                    false => match entry.selected {
                        true => Some(Color::Black),
                        false => None,
                    }
                };
                let bg = match entry.selected {
                    true => Some(Color::Red),
                    false => None,
                };
                let number_style = {
                    let mut number_style = Style::default();
                    if let Some(fg) = fg {
                        number_style = number_style.fg(fg);
                    }
                    if let Some(bg) = bg {
                        number_style = number_style.bg(bg);
                    }
                    number_style
                };
                let block_style = {
                    let mut block_style = Style::default();
                    if let Some(bg) = bg {
                        block_style = block_style.bg(bg);
                    }
                    block_style
                };

                let hour_str = format!("{}{}", row.hour, if row.hour < 12 { "AM" } else { "PM" });
                let block = Block::default()
                        .title(&hour_str)
                        .borders(border::TOP)
                        .title_style(number_style)
                        .border_style(number_style)
                        .style(block_style);
                block.draw(&rect, buffer);

                let mut style = Style::default();
                if entry.selected {
                    style = style.fg(Color::Black).bg(Color::Red);
                }
                for (line, sy) in entry.summary.lines().zip((rect.y + 1)..area.height) {
                    buffer.set_stringn(rect.x + 1, sy, line, (rect.width - 2) as usize, &style);
                }
            }
        }
    }
}


