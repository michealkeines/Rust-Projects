use std::io::prelude::*;
use std::fs::File;
use std::env;

const BYTES_PER_LINE: usize = 16;

fn main() {
    let arg1 = env::args().nth(1);

    let fname = arg1.expect("Usage: hex_dump <file>");

    let mut f = File::open(&fname).expect("Unable to open file.");
    let mut pos = 0;
    let mut val = 0;
    let mut buffer: Vec<u8> = vec![];
    let _ = f.read_to_end(&mut buffer);
    let mut current = 0;

    for byte in &buffer {
        if pos == 0 {
            print!("[{:08x}] ", val);
        }

        print!("{:02x} ", byte);

        pos += 1;
        current += 1;
        
        if *&buffer.len() == current as usize {
            let rem = 16 - pos;
            for _ in 0..rem {
                print!("00 ");
            }
            for i in (current - rem)..(current as usize) {
                let byte = &buffer[i];
                match byte {
                    0x21..=0x7e => print!("{}", *byte as char),
                    _ => print!(".")
                }
            }
        }

        if pos == BYTES_PER_LINE {
            for i in (current - BYTES_PER_LINE)..(current as usize) {
                let byte = &buffer[i];
                match byte {
                    0x21..=0x7e => print!("{}", *byte as char),
                    _ => print!(".")
                }
            }
            println!();
            val += BYTES_PER_LINE;
            pos = 0;
        }
    }
}