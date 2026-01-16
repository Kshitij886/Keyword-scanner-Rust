#![allow(unused)]
use core::num;
use std::{
    env,
    error::Error,
    fs::File,
    io::{ self, Read, Write },
    process,
    string,
    sync::mpsc::{ Sender, channel },
    thread,
};

pub struct Arguments {
    threads: u16,
    file_path: String,
    keyword: String,
}
impl Arguments {
    fn new(args: &[String]) -> Result<Arguments, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        } else if args.len() > 4 {
            return Err("too many arguments");
        }
        let f = args[1].clone();
        if f.contains(".txt") {
            return Ok(Arguments { threads: 4, file_path: f.clone(), keyword: args[2].clone() });
        } else {
            println!(
                "Usage: -j to select how many threads you want 
                    \r\n    -h or -help to show this help message
                "
            );
            return Err("Help");
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let arguments = Arguments::new(&args).unwrap_or_else(|e| {
        if e.contains("help") {
            process::exit(0);
        } else {
            eprintln!("{} problem parsing arguments: {}", program, e);
            process::exit(0);
        }
    });
    let num_threads = arguments.threads;
    let keyword = arguments.keyword;
    let content = match fetch_text_from_file(arguments.file_path) {
        Ok(s) => s,
        Err(e) => {
            println!("{} error", e);
            process::exit(0);
        }
    };
    let (tx, rx) = channel();
    for i in 0..num_threads {
        let tx = tx.clone();
        let content = content.clone();
        let keyword = keyword.clone();
        thread::spawn(move || { scan(tx, content, keyword, num_threads, i) });
    }
    let mut out: Vec<u16> = vec![];
    drop(tx);
    for p in rx {
        out.push(p);
    }
    println!("Printing the lines with keyword : {}", keyword);
    for line in out {
        println!("{}", line + 1);
    }
}

fn fetch_text_from_file(file_name: String) -> Result<Vec<String>, &'static str> {
    println!("{}", file_name);
    if let Ok(mut file) = File::open(file_name) {
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap_or_else(|error| {
            println!("{} error parsing to string", error);
            process::exit(0);
        });
        let line_text: Vec<String> = content
            .lines()
            .map(|s| s.to_string())
            .collect();
        println!("{:?}", line_text);
        return Ok(line_text);
    } else {
        return Err("Invalid file path");
    }
}
fn scan(tx: Sender<u16>, content: Vec<String>, keyword: String, num_thread: u16, i:u16) {
    let mut  start : u16 = i +1;
    let keyword = &keyword;
    loop {
        if content[start as usize].contains(keyword) {
            print!("Found \n");
            io::stdout().flush();
            tx.send(start);
        }
        if(content.len() as u16 - start as u16) <= num_thread {
            break;
        }
        start += num_thread;
    }
}
