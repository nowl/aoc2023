pub use clap::Parser;
use std::fs::File;
use std::io::{self, prelude::*};
use std::path::{Path, PathBuf};

pub fn read_as_lines(path: &Path) -> io::Result<Vec<String>> {
    let mut file = File::open(path)?;

    let mut buf = String::new();
    file.read_to_string(&mut buf)?;

    let mut lines = buf.split("\n").map(|s| s.to_owned()).collect::<Vec<_>>();

    if lines.last().map_or(false, |v| v == "") {
        lines.remove(lines.len() - 1);
    }

    Ok(lines)
}

pub fn read_line() -> io::Result<()> {
    // read line
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    handle.read_line(&mut buffer)?;

    Ok(())
}

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(short, long)]
    pub file: PathBuf,
}

#[macro_export]
macro_rules! cap_name_parse {
    ($capture:ident, $name:expr) => {
        $capture
            .name($name)
            .ok_or(anyhow!("regex capture error"))?
            .as_str()
            .parse()
    };
}

#[macro_export]
macro_rules! cap_name_str {
    ($capture:ident, $name:expr) => {
        $capture
            .name($name)
            .ok_or(anyhow!("regex capture error"))?
            .as_str()
    };
}

pub fn pause_enter() {
    let mut buf = String::new();
    let _ = io::stdin().read_line(&mut buf);
}
