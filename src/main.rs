use std::env;
use std::io::{self, Write, Read};
use std::fs::File;
//use std::io::prelude::*;

//#[cfg(unix)]
//use std::os::unix::fs::MetadataExt;

#[cfg(linux)]
use std::os::linux::fs::MetadataExt;

#[cfg(target_os="macos")]
use std::os::macos::fs::MetadataExt;


fn main() {
    let args: Vec<String> = env::args().collect();

    for file in &args[1..] {
        read_file(file);
    }
}

fn read_file(file: &String) {
    let mut buffersize: usize = 1024;
    match File::open(file) {
        Ok(mut handle) => {
            if let Ok(meta) = handle.metadata() {
                let blk = meta.st_blksize();
                if blk < std::usize::MAX as u64 {
                    buffersize = blk as usize;
                }
            }

            let mut buffer = vec![0; buffersize];

            while let Ok(n) = handle.read(&mut buffer) {
                if n == 0 {
                    break;
                };
                if let Err(e) = io::stdout().write_all(&buffer) {
                    eprintln!("{}", e);
                }
            }
        }
        Err(error) => {
            eprintln!("{}", error);
        }
    }
}
