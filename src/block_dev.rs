use std::io;
use std::fs;
use std::str;
use std::ffi::OsString;

struct BlockDevice {
    name: OsString,
    major: u32, /* major device number is a 12-bit int */
}

fn get_major_dev_num(name: OsString) -> Result<u32, io::Error> {
    let mut devno_path: OsString = OsString::from("/sys/block/");
    devno_path.push(name);
    devno_path.push("/dev");
    let s = match fs::read(devno_path) {
        Ok(contents) => contents,
        Err(e) => return Err(e)
    };
    let i = s.iter().position(|&r| r == 58)
        .ok_or(io::Error::from(io::ErrorKind::InvalidData))?;
    let maj = &s[..i];
    let maj = match str::from_utf8(maj) {
        Ok(m) => m,
        Err(_) => return Err(io::Error::from(io::ErrorKind::InvalidData)),
    };
    let maj: u32 = match maj.parse() {
        Ok(m) => m,
        Err(_) => return Err(io::Error::from(io::ErrorKind::InvalidData)),
    };
    Ok(maj)
}

fn load_block_dev_info(ent: fs::DirEntry) -> Result<BlockDevice, io::Error> {
    let name = ent.file_name();
    let mut dev = BlockDevice {
        name: name.clone(),
        major: 0,
    };
    dev.major = get_major_dev_num(name)?;
    return Ok(dev);
}

pub fn process_block_device(ent: fs::DirEntry) {
    let dev = match load_block_dev_info(ent) {
        Ok(d) => d,
        Err(e) => {
            println!("Error: {:?}", e);
            return;
        },
    };
    println!("device: {:?}, major: {}", dev.name, dev.major);
}
