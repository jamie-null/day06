use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead,BufReader};
use std::iter::FromIterator;

fn main() -> Result<(), Box<dyn Error>> {
    let raw = File::open("./input.txt")?;
    let buf = BufReader::new(raw);
    let mut s: HashSet<char> = HashSet::new();
    let mut groups = Vec::new();
    let mut sum = 0;
    let mut start = true;
    for line in buf.lines() {
        let line = line.unwrap();
        if line == "" {
            sum += s.len();
            groups.push(s);
            s = HashSet::new();
            start = true;
        } else {
            if start {
                start = false;
                line.chars().for_each(|c| {s.insert(c); return});
            } else {
                let person: HashSet<char> = HashSet::from_iter(line.chars());
                s.retain(|a| person.contains(a));
            }
        }
    }
    if !s.is_empty() {
        sum += s.len();
        groups.push(s);
    }
    println!("{}",sum);
    return Ok(());
}
