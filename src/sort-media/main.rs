extern crate chrono;
extern crate rexif;

use anyhow;
use chrono::NaiveDateTime;
use fs::File;
use getopts::Options;
use glob::glob;
use glob::Paths;
use rexif::ExifTag::DateTime as ExifDateTime;
use std::env;
use std::fs;
use std::io::prelude::*;
use std::io::SeekFrom;

fn get_date_from_photo(filename: &str) -> Option<NaiveDateTime> {
    let exif = rexif::parse_file(&filename).ok()?;

    for entry in &exif.entries {
        if entry.tag == ExifDateTime {
            return NaiveDateTime::parse_from_str(&entry.value_more_readable, "%Y:%m:%d %H:%M:%S")
                .ok();
        }
    }

    None
}

fn get_date_from_video(filename: &str) -> anyhow::Result<NaiveDateTime> {
    let mut bytes: [u8; 24] = [0; 24];
    let mut file = File::open(&filename)?;
    file.seek(SeekFrom::End(-24))?;
    file.read(&mut bytes)?;
    let date_string = String::from_utf8(bytes.to_vec())?;
    Ok(NaiveDateTime::parse_from_str(
        &date_string,
        "%Y-%m-%dT%H:%M:%S%Z",
    )?)
}

fn sort_videos(entries: Paths) -> anyhow::Result<()> {
    for entry in entries {
        match entry?.to_str() {
            // Get a file entry or just continue
            Some(f) => {
                let date = get_date_from_video(f).unwrap();
                let new_name = date.format("VID_%Y%m%d_%H%M%S.mov").to_string();
                fs::rename(f, &new_name)?;
                eprintln!("moved file to {}", &new_name);
            }

            None => continue,
        }
    }

    Ok(())
}

fn sort_photos(entries: Paths) -> anyhow::Result<()> {
    for entry in entries {
        match entry?.to_str() {
            Some(f) => {
                let date = get_date_from_photo(f).unwrap();
                let new_name = date.format("IMG_%Y%m%d_%H%M%S.jpg").to_string();
                fs::rename(f, &new_name)?;
                eprintln!("moved file to {}", &new_name);
            }

            None => continue,
        }
    }

    Ok(())
}

fn sort() -> anyhow::Result<()> {
    sort_photos(glob("*.[jJ][pP]*[gG]")?)?;
    sort_videos(glob("*.[mM][pPoOkK][vV~]")?)?;
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    let program_name = args[0].clone();

    let mut opts = Options::new();

    opts.optflag("h", "help", "show these options");
    let matches = opts.parse(&args[1..])?;
    let help = matches.opt_present("h");

    if help {
        let brief = format!("Usage: {} [options]", program_name);
        print!("{}", opts.usage(&brief));
    } else {
        return sort();
    }

    Ok(())
}

#[test]
fn finds_metadata_in_video() -> Result<NaiveDateTime, anyhow::Error> {
    get_date_from_video("test.mov")
}
