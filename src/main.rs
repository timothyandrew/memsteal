use std::{
    env,
    fs::{self, File},
    io::Seek,
    io::{Error, Read},
};

fn read_memory(pid: &str, offset: u64) -> Result<(), Error> {
    let file = format!("/proc/{}/mem", pid);
    let mut file = File::open(file)?;
    file.seek(std::io::SeekFrom::Start(offset))?;

    let mut buf = [0 as u8; 1024];
    let mut buf_string = String::new();
    let mut count = 0;

    let n = file.read(&mut buf)?;

    for byte in &buf[..n] {
        print!("{:02X} ", byte);

        if *byte > 33 && *byte < 127 {
            buf_string.push(*byte as char);
        } else {
            buf_string.push('.');
        }
        count = count + 1;

        if count >= 24 {
            println!(" {}", buf_string);
            buf_string.clear();
            count = 0;
        }
    }

    println!(" {}", buf_string);

    Ok(())
}

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();

    if let [_, pid, offset] = &args[..] {
        let offset = offset.strip_prefix("0x").unwrap_or(offset);

        if let Ok(offset) = u64::from_str_radix(offset, 16) {
            read_memory(pid, offset)?;
        } else {
            eprintln!("ERROR: offset should be a hex number");
            std::process::exit(1);
        };
    } else {
        eprintln!("usage: memsteal <pid> <hex_offset>");
        std::process::exit(1);
    }

    Ok(())
}
