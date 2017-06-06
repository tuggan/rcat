use std::env;
use std::io::{self, Write};
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;


fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", &args[1..]);

    for file in &args[1..] {
        readfile(file);
    }
}

fn readfile(file: &String) {
    const BUFFERSIZE: usize = 1024;
    let fd = File::open(file);
    if fd.is_ok() {

        let mut fdu = fd.unwrap();

        let metadata = fdu.metadata().unwrap();

        if metadata.is_file() {
            println!("Reading file {}", file);
            let mut buffer = [0; BUFFERSIZE];

            loop {
                let rstat = fdu.read(&mut buffer);
                if rstat.is_ok() {
                    io::stdout().write(&buffer);
                    
                    if rstat.unwrap() < BUFFERSIZE {
                        break;
                    }
                } else {
                    break;
                }
            }
            //let mut buf_reader = BufReader::new(fdu);
        }

    } else {
        println!("{}: No such file!", file);
    }
}
