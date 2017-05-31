use std::process::Command;
use std::path::Path;
use std::fs;
use termion::screen::ToAlternateScreen;

pub fn edit(path: &Path) {
    if let Some(dir) = path.parent() {
        fs::create_dir_all(dir).unwrap();
    }

    Command::new("vim").arg(path).spawn().unwrap().wait().unwrap();
    print!("{}", ToAlternateScreen);
}

