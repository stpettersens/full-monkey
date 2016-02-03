/*
    Full Monkey.
    Generic preprocessor tool.

    Copyright 2016 Sam Saint-Pettersen.
    Released under the MIT/X11 License.
*/

extern crate clioptions;
extern crate regex;
use clioptions::CliOptions;
use regex::Regex;
use std::io::{BufRead, BufReader, Write};
use std::fs::File;
use std::process::exit;

fn preprocess(input: &str, output: &str, conditions: &str, verbose: bool) {
    let mut loc: Vec<String> = Vec::new();
    let mut preprocessed: Vec<String> = Vec::new();
    let mut cond = String::new();
    let mut set_conds: Vec<String> = Vec::new();
    let mut prefixed: Vec<String> = Vec::new();
    let mut prefixes: Vec<String> = Vec::new();
    let mut in_pp = false;

    let conditions = conditions.split(",");
    for sc in conditions {
        set_conds.push(sc.to_string());
    }

    let f = File::open(input).unwrap();
    let file = BufReader::new(&f);
    for l in file.lines() {
        let l = l.unwrap();

        // Process prefix...
        let mut p = Regex::new("#prefix (.*) with (.*)").unwrap();
        if p.is_match(&l) {
            for cap in p.captures_iter(&l) {
                prefixed.push(cap.at(1).unwrap().to_string());
                prefixes.push(cap.at(2).unwrap().to_string());   
            }
            continue;
        }
        // Process conditional (if/else/elseif)...
        p = Regex::new("#[else]*[if]* (.*)").unwrap();
        if p.is_match(&l) {
            for cap in p.captures_iter(&l) {
                cond = cap.at(1).unwrap().to_string();
                in_pp = true; 
            }
            continue;
        }

        // Process end block...
        p = Regex::new("#[fi|endif]").unwrap();
        if p.is_match(&l) {
            in_pp = false;
            continue;
        }
        // Push relevant LoC to vector...
        for sc in set_conds.clone() {
            if in_pp && cond == sc {
                preprocessed.push(l.to_string());
            }
        }
        if !in_pp {
            preprocessed.push(l.to_string());
            continue;
        }
    }
    // Do any alterations:
    for line in preprocessed {
        let mut fl = line;
        for (i, p) in prefixed.iter().enumerate() {
            let r = Regex::new(&regex::quote(&p)).unwrap();
            let repl = format!("{}{}", &prefixes[i], &p);
            fl = r.replace_all(&fl, &repl[..]);   
        }
        loc.push(fl);  
    }
    loc.push(String::new());

    if verbose {
        println!("Preprocessing {} --> {}", input, output);
    }

    let mut w = File::create(output).unwrap();
    let _ = w.write_all(loc.join("\n").as_bytes());
}

fn display_version() {
    println!("Full Monkey v. 0.1");
    println!(r"     __");
    println!(r"w  c(..)o   (");
    println!(r" \__(-)    __)");
    println!(r"     /\   (");
    println!(r"    /(_)___)");
    println!(r"   w /|");
    println!(r"    | \");
    println!(r"   m  m");
    println!("\nMonkey appears courtesy of ejm97:");
    println!("http://www.ascii-art.de/ascii/mno/monkey.txt");
    exit(0);
}

fn display_error(program: &str, error: &str) {
    println!("Error: {}.", error);
    display_usage(&program, -1);
}

fn display_usage(program: &str, exit_code: i32) {
    println!("\nFull Monkey.");
    println!("Generic preprocessor tool.");
    println!("\nCopyright 2016 Sam Saint-Pettersen.");
    println!("Released under the MIT/X11 License.");
    println!("\n{} -f|--file <input> [-c|--condition <condition(s)>] -o|--out <output>", program);
    println!("[-l|--verbose][-h|--help | -v|--version]");
    println!("\n-f|--file: File to run preprocessing on.");
    println!("-c|--conditon: Comma delimited list of conditon(s) to apply.");
    println!("-o|--out: File to output preprocessed LoC to.");
    println!("-l|--verbose: Display output to console on process.");
    println!("-h|--help: Display this help information and exit.");
    println!("-v|--version: Display program versiona and exit.");
    exit(exit_code);
}

fn main() {
    let cli = CliOptions::new("fm");
    let program = cli.get_program();
    let mut input = String::new();
    let mut output = String::new();
    let mut conditions = String::new();
    let mut verbose = false;
    if cli.get_num() > 1 {
        for (i, a) in cli.get_args().iter().enumerate() {
            match a.trim() {
                "-h" | "--help" => display_usage(&program, 0),
                "-v" | "--version" => display_version(),
                "-f" | "--file" => input = cli.next_argument(i),
                "-c" | "--condition" => conditions = cli.next_argument(i),
                "-o" | "--out" => output = cli.next_argument(i),
                "-l" | "--verbose" => verbose = true,
                _ => continue,
            }
        }

        if input.is_empty() {
            display_error(&program, "No input specified");
        }

        if output.is_empty() {
            display_error(&program, "No output specified");
        }

        preprocess(&input, &output, &conditions, verbose);
    }
    else {
        display_error(&program, "No options specified");
    }
}
