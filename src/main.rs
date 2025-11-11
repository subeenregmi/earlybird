use std::time::{self, UNIX_EPOCH};

fn main() {
    let now = time::SystemTime::now().duration_since(UNIX_EPOCH);

    match now {
        Ok(n) => println!("The time now is {}", n.as_secs()),
        Err(_) => println!("The time is not valid"),
    }
}
