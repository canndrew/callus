use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::widgets::Widget;
use xdg;

use chrono::naive::datetime::NaiveDateTime;
use chrono::Timelike;

use std::sync::Arc;

use self::CalendarView::*;
use ::month::MonthView;
use ::week::WeekView;
use ::database::Database;

enum CalendarView {
    Year,
    Month,
    Week,
}

pub struct Calendar {
    view: CalendarView,
    month_view: MonthView,
    week_view: WeekView,
}

impl Calendar {
    pub fn new(datetime: NaiveDateTime) -> Calendar {
        let xdg_dirs = xdg::BaseDirectories::with_prefix("callus").unwrap();
        let path = match xdg_dirs.find_data_file("db") {
            Some(path) => path,
            None => xdg_dirs.place_data_file("db").unwrap(),
        };
        let db = Arc::new(Database::open(path));
        Calendar {
            view: CalendarView::Month,
            month_view: MonthView::new(db.clone(), datetime.date()),
            week_view: WeekView::new(db.clone(), datetime.date(), datetime.hour() as u8),
        }
    }

    pub fn next_view(&mut self) -> bool {
        self.view = match self.view {
            Year => Month,
            Month => {
                self.week_view.set_date(self.month_view.get_date());
                Week
            },
            Week => Week,
        };
        false
    }

    pub fn prev_view(&mut self) -> bool {
        self.view = match self.view {
            Year => Year,
            //Month => Year,
            Month => Month,
            Week => {
                self.month_view.set_date(self.week_view.get_date());
                Month
            },
        };
        false
    }

    pub fn up(&mut self) -> bool {
        match self.view {
            Month => {
                self.month_view.up()
            },
            Week => {
                self.week_view.up()
            },
            _ => unimplemented!(),
        }
    }

    pub fn down(&mut self) -> bool {
        match self.view {
            Month => {
                self.month_view.down()
            },
            Week => {
                self.week_view.down()
            },
            _ => unimplemented!(),
        }
    }

    pub fn right(&mut self) -> bool {
        match self.view {
            Month => {
                self.month_view.right()
            },
            Week => {
                self.week_view.right()
            },
            _ => unimplemented!(),
        }
    }

    pub fn left(&mut self) -> bool {
        match self.view {
            Month => {
                self.month_view.left()
            },
            Week => {
                self.week_view.left()
            },
            _ => unimplemented!(),
        }
    }

    pub fn enter(&mut self) -> bool {
        match self.view {
            Month => {
                self.month_view.enter()
            },
            Week => {
                self.week_view.enter()
            },
            _ => unimplemented!(),
        }
    }
}

impl Widget for Calendar {
    fn draw(&self, area: &Rect, buffer: &mut Buffer) {
        match self.view {
            Month => {
                self.month_view.draw(area, buffer)
            },
            Week => {
                self.week_view.draw(area, buffer)
            },
            _ => unimplemented!(),
        }
    }
}


