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
    let fd = File::open(file);
    if fd.is_ok() {
        println!("{}: File exists!", file);

        let mut fdu = fd.unwrap();

        let metadata = fdu.metadata().unwrap();

        if metadata.is_file() {
            println!("Reading file {}", file);
            let mut buffer = [0; 1024];

            fdu.read(&mut buffer);

            io::stdout().write(&buffer);
            //let mut buf_reader = BufReader::new(fdu);
        }

    } else {
        println!("{}: No such file!", file);
    }
}
