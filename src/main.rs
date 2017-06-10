/*
   Copyright 2017 Dennis Vesterlund

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
*/

use std::env;
use std::io::{self, Write, Read};
use std::fs::File;

mod file_helper;

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

            if let Ok(buffs) = file_helper::blocksize_as_usize(file) {
                buffersize = buffs;
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
