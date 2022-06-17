use std::{thread, time};
use winput::{Vk};

fn main() {
    
    loop{
        let sleep_time = time::Duration::from_secs(180);
        thread::sleep(sleep_time);
        winput::send(Vk::Tab); 
    }
}