#![allow(dead_code)]
use clap::Parser;
mod chapter_2;
use crate::chapter_2::guessing_game;
mod chapter_3;
use crate::chapter_3::all as chapter_3_all;
mod chapter_4;
use crate::chapter_4::all as chapter_4_all;
mod chapter_5;
use crate::chapter_5::all as chapter_5_all;
mod chapter_6;
use crate::chapter_6::all as chapter_6_all;
// For chapter_7 see the 'restaurant' project
mod chapter_8;
use crate::chapter_8::all as chapter_8_all;
mod chapter_9;
use crate::chapter_9::all as chapter_9_all;
mod chapter_10;
use crate::chapter_10::all as chapter_10_all;
mod chapter_11;
use crate::chapter_11::all as chapter_11_all;
mod chapter_13;
use crate::chapter_13::all as chapter_13_all;
mod chapter_15;
use crate::chapter_15::all as chapter_15_all;
mod chapter_16;
use crate::chapter_16::all as chapter_16_all;
mod chapter_17;
use crate::chapter_17::all as chapter_17_all;
mod chapter_18;
use crate::chapter_18::all as chapter_18_all;
#[derive(Parser,Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short,long,default_value_t=false)]
    interactive: bool,
}

fn main() {
    let args = Cli::parse();
    let interactive = args.interactive;
    println!("interactive {interactive}");
    // Chapter 2
    if args.interactive { guessing_game(); }
    chapter_3_all(args.interactive);
    chapter_4_all();
    chapter_5_all();
    chapter_6_all();
    chapter_8_all();
    chapter_9_all();
    chapter_10_all();
    chapter_11_all();
    // chapter_12 is in the minigrep project
    chapter_13_all();
    chapter_15_all();
    chapter_16_all();
    chapter_17_all();
    chapter_18_all();
}
