mod block_dev;

use std::fs;

use block_dev::process_block_device;

fn read_sysfs() {
    let dirents = fs::read_dir("/sys/block/").expect("Can't read sysfs");
    for entry in dirents {
        match entry {
            Ok(ent) => process_block_device(ent),
            Err(err) => println!("Error: {}", err),
        };
    }
}

fn main() {
    read_sysfs();
}
