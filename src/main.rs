use std::{
    fs::OpenOptions,
    io::Write,
    time::{Duration, SystemTime},
};

use chrono::{DateTime, Utc};
use clipboard::{ClipboardContext, ClipboardProvider};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 3 {
        eprintln!(
            "ERR: Invalid syntax! Usage: {} [output_file] [should_export_time]",
            args[0]
        );
        std::process::exit(1);
    }

    let output_file_path = &args[1];
    let should_export_time = &args[2];

    let should_export_time = match should_export_time.as_str() {
        "true" => true,
        _ => false,
    };

    let mut output_file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(output_file_path)
        .expect("ERR: Failed to open output file!");

    let mut clipboard: ClipboardContext =
        ClipboardProvider::new().expect("ERR: Failed to create ClipboardProvider!");
    let mut prev_content = clipboard
        .get_contents()
        .expect("ERR: Failed to get initial clipboard contents!");

    loop {
        let current_content = clipboard
            .get_contents()
            .expect("ERR: Failed to get clipboard content in loop!");

        if current_content != prev_content {
            prev_content = current_content.clone();

            println!("New item in clipboard! '{}'", current_content);

            let mut current_content = if should_export_time {
                let st_now = SystemTime::now();
                let dt_now_utc: DateTime<Utc> = st_now.into();
                format!(
                    "[{}]: '{}'",
                    dt_now_utc.format("%Y/%m/%d %H:%M:%S"),
                    current_content
                )
            } else {
                format!("'{}'", current_content)
            };

            current_content += "\n";
            output_file
                .write_all(current_content.as_bytes())
                .expect("ERR: Failed to write to output file!");
        }

        std::thread::sleep(Duration::from_millis(50));
    }
}
