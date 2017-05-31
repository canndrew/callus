use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::widgets::{border, Widget, Block};
use tui::style::{Color, Modifier, Style};

use chrono::naive::date::NaiveDate;
use chrono::offset::local::Local;
use chrono::Datelike;

use std::sync::Arc;

use ::{DAY_NAMES, MONTH_NAMES, one_day};
use ::database::Database;

pub struct MonthView {
    selected_date: NaiveDate,
    db: Arc<Database>,
}

impl MonthView {
    pub fn new(db: Arc<Database>, date: NaiveDate) -> MonthView {
        //let cur_day = date.weekday().num_days_from_monday();
        //let cur_week = (date.day0() + cur_day) / 7;
        MonthView {
            selected_date: date,
            db: db,
        }
    }

    pub fn set_date(&mut self, date: NaiveDate) {
        self.selected_date = date;
    }

    pub fn get_date(&self) -> NaiveDate {
        self.selected_date
    }

    pub fn up(&mut self) -> bool {
        self.selected_date = self.selected_date - one_day() * 7;
        false
    }

    pub fn down(&mut self) -> bool {
        self.selected_date = self.selected_date + one_day() * 7;
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
        self.db.edit_day(date);
        true
    }

    pub fn draw(&self, area: &Rect, buffer: &mut Buffer) {
        let title = format!("{} {}", MONTH_NAMES[self.selected_date.month0() as usize], self.selected_date.year());
        buffer.set_string((area.width - title.len() as u16) / 2, 0, &title, &Style::default().fg(Color::Yellow).modifier(Modifier::Bold));

        let first_day_of_month = self.selected_date - (one_day() * self.selected_date.day0() as i32);
        let first_day = first_day_of_month - (one_day() * first_day_of_month.weekday().num_days_from_monday() as i32);

        let w = area.width - 1;
        let h = area.height - 4;
        for day_num in 0..7 {
            let x = 1 + (1 + day_num * 2) * w / 14;
            buffer.set_string(x, 2, DAY_NAMES[day_num as usize], &Style::default().modifier(Modifier::Bold));
            for week_num in 0..6 {
                let rect = Rect {
                    x: day_num * w / 7,
                    y: 3 + (week_num * h / 6),
                    width: ((day_num + 1) * w / 7) - (day_num * w / 7) + 1,
                    height: ((week_num + 1) * h / 6) - (week_num * h / 6) + 1,
                };

                let today = Local::now().naive_local().date();
                let day = first_day + one_day() * (7 * week_num + day_num) as i32;

                let day_of_month = match day.month0() == self.selected_date.month0() {
                    true => format!("{}", day.day()),
                    false => format!("{} {}", day.day(), MONTH_NAMES[day.month0() as usize]),
                };
                let fg = match day == today {
                    true => Some(Color::Yellow),
                    false => match day == self.selected_date {
                        true => Some(Color::Black),
                        false => match day.month0() == self.selected_date.month0() {
                            true => None,
                            false => Some(Color::Red),
                        },
                    }
                };
                let bg = match day == self.selected_date {
                    true => Some(Color::Red),
                    false => None,
                };
                let number_style = {
                    let mut number_style = Style::default();
                    if day.month0() == self.selected_date.month0() {
                        number_style = number_style.modifier(Modifier::Bold);
                    }
                    if let Some(fg) = fg {
                        number_style = number_style.fg(fg);
                    }
                    if let Some(bg) = bg {
                        number_style = number_style.bg(bg);
                    }
                    number_style
                };
                let line_style = {
                    let mut line_style = Style::default();
                    if let Some(bg) = bg {
                        line_style = line_style.bg(bg);
                    }
                    line_style
                };

                buffer.set_string(rect.x + 1, rect.y + 1, &day_of_month, &number_style);
                let block = Block::default().borders(border::ALL).style(line_style);
                block.draw(&rect, buffer);

                let mut style = Style::default();
                if day == self.selected_date {
                    style = style.fg(Color::Black).bg(Color::Red);
                }
                let mut summary = self.db.get_day(day);
                for hour in 0..24 {
                    summary.push_str(&self.db.get_hour(day, hour));
                }
                for (line, y) in summary.lines().zip((rect.y + 2)..(rect.y + 2 + rect.height - 3)) {
                    buffer.set_stringn(rect.x + 1, y, line, rect.width as usize - 1, &style);
                }
            }
        }
    }
}

