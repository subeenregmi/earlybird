use std::{
    fs::{File, create_dir},
    io::{Read, Write},
    path::Path,
};

use chrono::{DateTime, Datelike, Local, Month, Weekday};

#[derive(Debug)]
struct LoggedTimes {
    times: Vec<DateTime<Local>>,
}

const DATA_DIR: &str = "/home/subeen/.local/share/earlybird/";
const DATA_FILE: &str = "data";

fn main() {
    let dir_path = Path::new(DATA_DIR);

    let dir_exists = match dir_path.try_exists() {
        Ok(b) => b,
        Err(e) => panic!("error trying to check if dir path exists, {}", e),
    };

    if !dir_exists {
        if let Err(e) = create_dir(dir_path) {
            panic!("error trying to create new dir, {}", e)
        }
    }
    let mut file: File;

    let mut file_path_str = String::from(DATA_DIR);
    file_path_str.push_str(DATA_FILE);

    let file_path = Path::new(&file_path_str);

    let file_exists = match file_path.try_exists() {
        Ok(b) => b,
        Err(e) => panic!("error trying to check if file exists {}", e),
    };

    if file_exists {
        file = match File::options().read(true).write(true).open(file_path) {
            Ok(f) => f,
            Err(e) => panic!("error trying to open file, {}", e),
        }
    } else {
        file = match File::create_new(file_path) {
            Ok(f) => f,
            Err(e) => panic!("error trying to create new file, {}", e),
        }
    }

    let mut times_buf = String::new();

    if let Err(e) = file.read_to_string(&mut times_buf) {
        panic!("error trying to read file, {}", e)
    }

    let mut logged_times = LoggedTimes { times: Vec::new() };

    for time in times_buf.split("\n") {
        if time == "" {
            continue;
        }
        match time.parse::<DateTime<Local>>() {
            Ok(t) => logged_times.times.push(t),
            Err(e) => panic!("error parsing time entry, {}, {}", time, e),
        }
    }

    let mut current_entry = Local::now().to_string();
    current_entry.push('\n');

    if let Err(e) = file.write(current_entry.as_bytes()) {
        panic!("error trying to write to file, {}", e)
    }

    if let Err(e) = file.flush() {
        panic!("error trying to flush file contents, {}", e)
    }

    let current_time = Local::now();

    let month_num = match u8::try_from(current_time.month0()) {
        Ok(n) => n,
        Err(e) => panic!("error trying to turn month into u8, {}", e),
    };

    let month = match Month::try_from(month_num) {
        Ok(m) => m,
        Err(e) => panic!("error trying to turn month u8 to month, {}", e),
    };

    let day_num = match u8::try_from(current_time.day()) {
        Ok(n) => n,
        Err(e) => panic!("error trying to turn day into u8, {}", e),
    };

    let header = format!("{} {}", month.name(), current_time.year());

    let left_padding = ((4 * 7) / 2) - (header.len() / 2);
    println!("{:<left_padding$}{}", "", header);

    let days: [Weekday; 7] = [
        Weekday::Mon,
        Weekday::Tue,
        Weekday::Wed,
        Weekday::Thu,
        Weekday::Fri,
        Weekday::Sat,
        Weekday::Sun,
    ];

    for day in days {
        print!("{} ", day)
    }

    println!();

    for i in 1..=current_time.num_days_in_month() {
        if i == day_num {
            print!("\x1b[1;91m{i:<4}\x1b[0m")
        } else {
            print!("{i:<4}");
        }
        if i % 7 == 0 {
            println!()
        }
    }

    println!()
}
