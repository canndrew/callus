use std::process::Command;
use std::path::Path;
use std::fs;
use termion::screen::ToAlternateScreen;

/*
pub fn edits(initial_text: &str) -> Option<String> {
    let mut file = tempfile::NamedTempFile::new().unwrap();
    file.write(initial_text.as_bytes()).unwrap();
    let path = file.path();

    let errcode = Command::new("vim").arg(path).spawn().unwrap().wait().unwrap();
    if !errcode.success() {
        return None;
    }

    let mut file = File::open(path).unwrap();
    let mut ret = String::new();
    file.read_to_string(&mut ret);
    Some(ret)
}

pub fn edit_field(datetime: NaiveDate
*/

pub fn edit(path: &Path) {
    if let Some(dir) = path.parent() {
        fs::create_dir_all(dir).unwrap();
    }

    Command::new("vim").arg(path).spawn().unwrap().wait().unwrap();
    print!("{}", ToAlternateScreen);
}

