mod misc;
mod sites;

use clap::{Arg, Command};
use misc::*;
use sites::*;
use std::env;
use std::fs::{File, self};
use std::io::{BufRead, self, Write};
use std::path::Path;

fn main() -> io::Result<()> {
    let matches = Command::new("WYT-DLP")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Wrapper for YT-DLP")
        .arg(
            Arg::new("STORED")
                .help("Sets the location of the stored URLS")
                .required(true),
        )
        .arg(
            Arg::new("STREAM")
                .short('s')
                .help("unused for now")
                .required(false),
        )
        .get_matches();

    update::main();

    let source_file = matches.get_one::<String>("STORED").unwrap();
    let mut out_dir = String::new();
    if cfg!(target_os = "windows") {
        if let Ok(user_profile) = env::var("USERPROFILE") {
            out_dir.push_str(&user_profile);
            out_dir.push_str("\\Videos\\downloaded_files");
        }
    } else {
        if let Ok(home) = env::var("HOME") {
            out_dir.push_str(&home);
            out_dir.push_str("/Videos/downloaded_files");
        }
    }
    if !out_dir.is_empty() && !dir_path_exists(&out_dir) {
        fs::create_dir_all(&out_dir).expect("Failed to create directory");
    }

    let media_file = File::open(source_file)?;
    let media_reader = io::BufReader::new(media_file);

    for line in media_reader.lines() {
        let current_line = line.expect("Unable to read line");
        println!("{:?}", current_line);
        let url_start = current_line.find("://").map_or(0, |pos| pos + 3);
        let url_end = current_line[url_start..]
            .find('/')
            .unwrap_or(current_line.len());

        let current_site = &current_line[url_start..url_start + url_end];
        match current_site {
            "www.pornhub.com" => video::main(current_line, &out_dir),
            "www.redgifs.com" => video::main(current_line, &out_dir),
            "www.twitch.tv" => println!("Site not implemented"),
            "www.youtube.com" => video::main(current_line, &out_dir),
            "youtu.be" => video::main(current_line, &out_dir),
            _ => println!("Site not implemented"),
        };
    }

    Ok(())
}

fn dir_path_exists(path: &str) -> bool {
    fs::metadata(path).is_ok()
}
