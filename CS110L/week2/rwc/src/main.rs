use std::env;
use std::process;
use std::fs::File; // For read_file_lines()
use std::io::{self, BufRead}; // For read_file_lines()

fn read_file_lines(filename : &String) -> Result<Vec<String>, io::Error> {
    let file = File::open(filename)?;
    let mut v = Vec::new();
    for line in io::BufReader::new(file).lines() {
        let cur_str = line?;
        v.push(cur_str);
    }
    Ok(v)
}

fn calc(v : &Vec<String>, word_cnt : &mut usize, char_cnt : &mut usize) {
    let mut res : String = String::new();
    for s in v {
        for c in s.chars() {
            res.push(c);
            *char_cnt += 1;
            if c == ' ' {
                *word_cnt += 1;
                res.clear();
            }
        }
        if !res.is_empty() {
            *word_cnt += 1;
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Too few arguments.");
        process::exit(1);
    }

    let filename = &args[1];
    
    let mut word_cnt : usize = 0;
    let mut char_cnt : usize = 0;

    let v = read_file_lines(&filename).unwrap();

    let line_cnt = v.len();

    calc(&v, &mut word_cnt, &mut char_cnt);

    char_cnt += line_cnt;

    println!("\t{}\t{}\t{} {}", line_cnt, word_cnt, char_cnt, filename);
}
