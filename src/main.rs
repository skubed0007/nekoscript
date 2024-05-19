pub mod build;
pub mod createproject;
pub mod run;
pub mod util;

use std::io::stdin;

use colored::Colorize;

use crate::{
    build::build,
    createproject::createproj,
    run::run,
    util::{clear, flush},
};

fn main() {
    clear();
    println!("{}", "NekoScript - NS".bold().blink().blue());
    println!(
        "{}",
        "Please Choose One\n1.) Build\n2.) New Project\n3.) Run".bold()
    );
    let mut chc = String::new();
    print!("{}", "> ".bold().bright_white());
    flush();
    stdin().read_line(&mut chc).unwrap();
    match chc.trim() {
        "1" => {
            clear();
            build();
        }
        "2" => {
            clear();
            createproj();
        }
        "3" => {
            clear();
            run();
        }
        _ => {
            println!("Invaid option!\nAre you typing the number corrospoding to the option and is the number valid\nshould be 1,2 or 3")
        }
    }
}
