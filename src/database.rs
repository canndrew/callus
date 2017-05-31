use chrono::naive::date::NaiveDate;
use chrono::Datelike;

use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::Read;
use std::collections::{hash_map, HashMap};
use std::sync::Mutex;

use ::editor;
use ::{MONTH_NAMES};

pub struct Database {
    location: PathBuf,
    days: Mutex<HashMap<NaiveDate, String>>,
    hours: Mutex<HashMap<(NaiveDate, u8), String>>,
}

impl Database {
    pub fn open(location: PathBuf) -> Database {
        println!("got dir: {:?}", location);
        Database {
            location: location,
            days: Mutex::new(HashMap::new()),
            hours: Mutex::new(HashMap::new()),
        }
    }

    fn day_filename(&self, day: NaiveDate) -> PathBuf {
        let mut path = self.location.clone();
        path.push(format!("{}", day.year()));
        path.push(format!("{}", MONTH_NAMES[day.month0() as usize]));
        path.push(format!("{}", day.day()));
        path.push("today.txt");
        path
    }

    fn hour_filename(&self, day: NaiveDate, hour: u8) -> PathBuf {
        let mut path = self.location.clone();
        path.push(format!("{}", day.year()));
        path.push(format!("{}", MONTH_NAMES[day.month0() as usize]));
        path.push(format!("{}", day.day()));
        path.push(format!("{}{}", hour, if hour < 12 { "AM" } else { "PM" }));
        path
    }

    pub fn get_day(&self, day: NaiveDate) -> String {
        let mut days = self.days.lock().unwrap();
        match days.entry(day) {
            hash_map::Entry::Occupied(oe) => oe.get().to_owned(),
            hash_map::Entry::Vacant(ve) => {
                let path = self.day_filename(day);
                ve.insert(load_entry(&path)).clone()
            },
        }
    }

    pub fn edit_day(&self, day: NaiveDate) {
        let path = self.day_filename(day);
        editor::edit(&path);
        let mut days = self.days.lock().unwrap();
        days.insert(day, load_entry(&path));
    }

    pub fn get_hour(&self, day: NaiveDate, hour: u8) -> String {
        let mut hours = self.hours.lock().unwrap();
        match hours.entry((day, hour)) {
            hash_map::Entry::Occupied(oe) => oe.get().to_owned(),
            hash_map::Entry::Vacant(ve) => {
                let path = self.hour_filename(day, hour);
                ve.insert(load_entry(&path)).clone()
            },
        }
    }

    pub fn edit_hour(&self, day: NaiveDate, hour: u8) {
        let path = self.hour_filename(day, hour);
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

