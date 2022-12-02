use std::path::Path;
use std::fs::File;
use std::io::BufRead;
use std::io::{self, BufReader};
use std::io::prelude::*;

fn read_file_buffer(filepath: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let f = File::open(filepath)?;
    let f = BufReader::new(f);

    for line in f.lines() {
        let line = line?;
        println!("{}", &line);
        
    }
    Ok(())
}

fn main() {

    let path = Path::new("./input");
    read_file_buffer(path);
}
