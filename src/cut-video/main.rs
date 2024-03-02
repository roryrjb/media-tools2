extern crate getopts;

use getopts::{HasArg, Occur, Options};
use std::env;
use std::process::Command;

fn main() {
    let program_name = env::args().next().unwrap();
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.optflag("h", "help", "show this message");
    opts.opt(
        "s",
        "start",
        "time to cut from",
        "TIME",
        HasArg::Yes,
        Occur::Optional,
    );
    opts.opt(
        "e",
        "end",
        "time to cut to",
        "TIME",
        HasArg::Yes,
        Occur::Optional,
    );
    opts.opt(
        "n",
        "name",
        "output filename",
        "PATH",
        HasArg::Yes,
        Occur::Optional,
    );

    let matches = opts.parse(&args[1..]).unwrap();

    if matches.opt_present("h") || !matches.opt_present("n") {
        let brief = format!("Usage: {} [options]", program_name);
        print!("{}", opts.usage(&brief));
    } else {
        let start_time = match matches.opt_str("s") {
            Some(arg) => arg,
            None => String::from("00:00:00"),
        };

        let end_time = match matches.opt_str("end") {
            Some(arg) => arg,
            None => String::from("23:59:59"),
        };

        let output_filename = matches.opt_get_default("n", String::from("blah")).unwrap();
        let input_file = &matches.free.first().unwrap();

        Command::new("ffmpeg")
            .args(&[
                "-ss",
                &start_time,
                "-to",
                &end_time,
                "-i",
                &input_file,
                "-c",
                "copy",
                &output_filename,
            ])
            .status()
            .expect("Failed to run 'ffmpeg' command.");
    }
}
