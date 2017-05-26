use chrono::naive::date::NaiveDate;
use chrono::Datelike;

use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::Read;
use std::collections::{hash_map, HashMap};
use std::sync::Mutex;

use ::editor;
use ::{DB_LOC, MONTH_NAMES};

pub struct Database {
    days: Mutex<HashMap<NaiveDate, String>>,
    hours: Mutex<HashMap<(NaiveDate, u8), String>>,
}

impl Database {
    pub fn open() -> Database {
        Database {
            days: Mutex::new(HashMap::new()),
            hours: Mutex::new(HashMap::new()),
        }
    }

    pub fn get_day(&self, day: NaiveDate) -> String {
        let mut days = self.days.lock().unwrap();
        match days.entry(day) {
            hash_map::Entry::Occupied(oe) => oe.get().to_owned(),
            hash_map::Entry::Vacant(ve) => {
                let path = day_filename(day);
                ve.insert(load_entry(&path)).clone()
            },
        }
    }

    pub fn edit_day(&self, day: NaiveDate) {
        let path = day_filename(day);
        editor::edit(&path);
        let mut days = self.days.lock().unwrap();
        days.insert(day, load_entry(&path));
    }

    pub fn get_hour(&self, day: NaiveDate, hour: u8) -> String {
        let mut hours = self.hours.lock().unwrap();
        match hours.entry((day, hour)) {
            hash_map::Entry::Occupied(oe) => oe.get().to_owned(),
            hash_map::Entry::Vacant(ve) => {
                let path = hour_filename(day, hour);
                ve.insert(load_entry(&path)).clone()
            },
        }
    }

    pub fn edit_hour(&self, day: NaiveDate, hour: u8) {
        let path = hour_filename(day, hour);
        editor::edit(&path);
        let mut hours = self.hours.lock().unwrap();
        hours.insert((day, hour), load_entry(&path));
    }

}

fn load_entry(path: &Path) -> String {
    if path.exists() {
        let mut s = String::new();
        let mut f = File::open(&path).unwrap();
        f.read_to_string(&mut s).unwrap();
        s
    } else {
        String::new()
    }
}

fn day_filename(day: NaiveDate) -> PathBuf {
    format!("{}/{}/{}/{}/today.txt",
        DB_LOC,
        day.year(),
        MONTH_NAMES[day.month0() as usize],
        day.day()).into()
}

fn hour_filename(day: NaiveDate, hour: u8) -> PathBuf {
    format!("{}/{}/{}/{}/{}{}.txt",
        DB_LOC,
        day.year(),
        MONTH_NAMES[day.month0() as usize],
        day.day(),
        hour,
        if hour < 12 { "AM" } else { "PM" }).into()
}

