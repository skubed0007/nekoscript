use crate::util::{flush, wait};
use colored::Colorize;
use regex::Regex;
use std::{
    fs::{self, File},
    io::{stdin, Read, Write},
    path::Path,
    process::{exit, Command},
};

pub fn run() {
    print!("{}", "File(tip : Drag And Drop it here) : ".bold().green());
    flush();
    let mut infl = String::new();
    stdin().read_line(&mut infl).unwrap();
    let infl = infl.trim().trim_matches('\'').trim_matches('\"');
    let infl = Path::new(infl);
    if infl.is_file() {
        println!("{}", "FOUND FILE!".yellow());
    } else {
        println!(
            "{}",
            r"FILE NOT FOUND\ARE YOU SURE THIS IS THE CORRECT PATH?".red()
        );
        wait();
        exit(1);
    }

    let mut codef = File::open(infl).expect("Unable to open the input file");
    let mut rawcode = String::new();
    codef
        .read_to_string(&mut rawcode)
        .expect("Unable to read the input file");

    // Normalize line endings to \n
    let rawcode = rawcode.replace("\r\n", "\n").replace("\r", "\n");

    let sepbynewlcode = rawcode.split('\n');

    let mut newrscode = String::new();
    let txtinecho = Regex::new(r#"echonl\("(.*?)"\);"#).expect("Invalid regex pattern");
    let vardecl = Regex::new(r#"may\s+(\w+)\s*=\s*(String!|Int!|Float!);"#).expect("Invalid regex pattern");
    let mutvardecl = Regex::new(r#"may\s+whole\s+(\w+)\s*=\s*(.*);"#).expect("Invalid regex pattern");
    let inputvarnm = Regex::new(r#"takein\(\*(\w+)\);"#).unwrap();
    let mut curindex = 1;

    for frc in sepbynewlcode {
        if frc.contains("echonl")
            && frc.ends_with(';')
            && frc.contains('(')
            && frc.contains(')')
        {
            if let Some(cap) = txtinecho.captures(frc) {
                let extracted_text = &cap[1];
                newrscode = format!("{}\nprintln!(\"{}\");", newrscode, extracted_text);
                curindex += 1;
            }
        } else if frc.trim().contains("may")
            && frc.trim().contains("=")
            && !frc.trim().contains("whole")
            && frc.contains(";")
        {
            if let Some(cap) = vardecl.captures(frc) {
                let varnm = cap.get(1).map_or("", |m| m.as_str());
                let vartype = cap.get(2).map_or("", |m| m.as_str());
                let vardelcrscode = match vartype {
                    "String!" => format!("let mut {} = String::new();", varnm),
                    "Int!" => format!("let {} = 0;", varnm),
                    "Float!" => format!("let {} = 0.0;", varnm),
                    _ => {
                        eprintln!("error - invalid code at line: {} : Code: {}\nAre you declaring variable the proper way?", curindex, frc);
                        exit(1);
                    }
                };
                newrscode = format!("{}\n{}", newrscode, vardelcrscode);
                curindex += 1;
            }
        } else if frc.trim().contains("may")
            && frc.trim().contains("=")
            && frc.trim().contains("whole")
            && frc.contains(";")
        {
            if let Some(cap) = mutvardecl.captures(frc) {
                let varnm = cap.get(1).map_or("", |m| m.as_str());
                let vartype = cap.get(2).map_or("", |m| m.as_str());
                let vardelcrscode = match vartype {
                    "String!" => format!("let mut {} = String::new();", varnm),
                    "Int!" => format!("let {} = 0;", varnm),
                    "Float!" => format!("let {} = 0.0;", varnm),
                    _ => {
                        eprintln!("error - invalid code at line: {} : Code: {}\nAre you declaring variable the proper way?", curindex, frc);
                        exit(1);
                    }
                };
                newrscode = format!("{}\n{}", newrscode, vardelcrscode);
                curindex += 1;
            }
        } else if frc.trim().contains("takein")
            && frc.contains(")")
            && frc.contains("(")
            && frc.contains(";")
        {
            if let Some(invnm) = inputvarnm.captures(frc) {
                let invn = invnm.get(1).map_or("_", |m| m.as_str());
                let inoutrscode = format!("std::io::stdin().read_line(&mut {}).unwrap();", invn);
                newrscode = format!("{}\n{}", newrscode, inoutrscode);
                curindex += 1;
            }
        } else if frc.trim() == "exit!;" {
            newrscode = format!("{}\nstd::process::exit(0);", newrscode);
        } else {
            eprintln!("error - invalid code at line: {} : Code: {}", curindex, frc);
            exit(1);
        }
    }

    let rscode = format!("fn main(){}\n{}\n{}", "{", newrscode, "}");
    println!("Generated Rust code: \n{}", rscode);

    let mut tmprsfile = File::create("./tmprs.rs").expect("Unable to create temporary Rust file");
    tmprsfile
        .write_all(rscode.as_bytes())
        .expect("Unable to write to temporary Rust file");

    // Run rustfmt
    let rustfmt_status = Command::new("rustfmt")
        .arg("./tmprs.rs")
        .status()
        .expect("Failed to run rustfmt");
    if !rustfmt_status.success() {
        eprintln!("rustfmt failed");
        fs::remove_file("./tmprs.rs").expect("Unable to remove temporary Rust file");
        exit(1);
    }

    // Compile tmprs.rs
    let rustc_status = Command::new("rustc")
        .arg("./tmprs.rs")
        .status()
        .expect("Failed to run rustc");
    if !rustc_status.success() {
        eprintln!("rustc failed");
        fs::remove_file("./tmprs.rs").expect("Unable to remove temporary Rust file");
        exit(1);
    }
    fs::remove_file("./tmprs.rs").expect("Unable to remove temporary Rust file");
    let frun_status = Command::new("./tmprs.exe")
        .status()
        .expect("Failed to run the compiled executable");
    if !frun_status.success() {
        eprintln!("Running the compiled executable failed");
        exit(1);
    }
    fs::remove_file("./tmprs.exe").expect("Unable to remove temporary Rust file");
    fs::remove_file("./tmprs.pdb").expect("Unable to remove temporary Rust file");
}
