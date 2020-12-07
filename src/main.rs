use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead,BufReader};

fn main() -> Result<(), Box<dyn Error>> {
    let raw = File::open("./input.txt")?;
    let buf = BufReader::new(raw);
    let mut s = HashSet::new();
    let mut groups = Vec::new();
    let mut sum = 0;
    for line in buf.lines() {
        let line = line.unwrap();
        if line == "" {
            sum += s.len();
            groups.push(s);
            s = HashSet::new();
        } else {
            line.chars().for_each(|c| {s.insert(c); return});
        }
    }
    if(!s.is_empty()){
        sum += s.len();
        groups.push(s);
    }
    println!("{}",sum);
    return Ok(());
}
