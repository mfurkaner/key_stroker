use std::{thread, time, io::{self, Write}};
use winput::{Vk};

use colored::Colorize;

fn main() {
    let the_key = Vk::Tab;
    let sleep_for_seconds = 180;

    print!("{}",format!("\n\tSimulating {:?} every {} seconds.",the_key, sleep_for_seconds).red().bold());
    io::stdout().flush().unwrap();

    loop{
        let sleep_time = time::Duration::from_secs(sleep_for_seconds);
        thread::sleep(sleep_time);
        winput::send(the_key); 
    }
}