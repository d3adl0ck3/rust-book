use std::{env,process};
use minigrep::Config;
fn main() {
    let old_school = env::var("OLD_SCHOOL").is_ok();
    if old_school {
        main_v1();
    } else {
        main_v2();
    }
}
fn main_v1() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build_v1(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}",config.query);
    println!("In file {}",config.file_path);
    if let Err(e) = minigrep::run_v1(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
fn main_v2() {

    let config = Config::build_v2(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}",config.query);
    println!("In file {}",config.file_path);
    if let Err(e) = minigrep::run_v2(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
