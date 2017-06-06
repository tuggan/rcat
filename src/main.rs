use std::env;
use std::io::{self, Write};
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;


fn main() {
    let args: Vec<String> = env::args().collect();

    for file in &args[1..] {
        readfile(file);
    }
}

fn readfile(file: &String) {
    let fd = File::open(file);
    if fd.is_ok() {

        let mut fdu = fd.unwrap();

        let metadata = fdu.metadata().unwrap();

        if metadata.is_file() {
            
            print_file(fdu)
            
        }

    } else {
        eprintln!("{}: No such file!", file);
    }
}


fn print_file(mut fd: File) {

    const BUFFERSIZE: usize = 1024;
    let mut buffer = [0; BUFFERSIZE];
    loop {
            let rstat = fd.read(&mut buffer);
            if rstat.is_ok() {
                io::stdout().write(&buffer);
                
                if rstat.unwrap() < BUFFERSIZE {
                    break;
                }
            } else {
                break;
            }
        }
}
