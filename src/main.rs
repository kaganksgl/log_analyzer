use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::process;

mod parser;

#[derive(Debug, Default)]

struct Stats {
    lines: u64,
    blank: u64,
    bytes: u64,
}

fn handle_line(line_no: u64, line: &str) {
    if line_no <= 10 {println!("{:>6} | {}", line_no, line);}
}

fn process<R: BufRead>(mut reader: R) -> io::Result<Stats> {
    let mut buf = String::new();
    let mut stats = Stats::default();
    loop {
        buf.clear();
        let n = reader.read_line(&mut buf)?;
        if n == 0 {break;}
        stats.bytes += n as u64;
        stats.lines += 1;

        let line = buf.trim_end();
        if line.is_empty() {stats.blank += 1; continue;}

        handle_line(stats.lines, line);
    }
    Ok(stats)    
}

fn main() -> io::Result<()> {

    let path= match env::args().nth(1) {
        Some(p) => p,
        None => {
            eprintln!("log_analyser <file>");
            process::exit(2);
        }
    };


    
    let file = match File::open(&path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("error: '{}' could not be opened: {}", path, e);
            process::exit(1);
        }
    };

    match process(BufReader::new(file)) {
        Ok(stats) => {
            eprintln!("\n{} - {} lines ({} empty lines), {} bytes", path, stats.lines, stats.blank, stats.bytes);
        }
        Err(e) => {
            eprintln!("Errorr: while reading '{}': {}", path, e);
            process::exit(1);
        }
    }
    Ok(())
}
