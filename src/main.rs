use std::{
    fmt::Debug,
    io::{stdout, Write},
    thread::sleep,
    time::{Duration, Instant},
};

use clap::Parser;

mod parser;
use codespan_reporting::{
    diagnostic::{Diagnostic, Label},
    files::SimpleFiles,
    term::{
        self,
        termcolor::{ColorChoice, StandardStream},
    },
};
use gears::Gear;
use parser::gearbox_parser;
use peg::{error::ParseError, str::LineCol};

mod gears;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    file: String,
    #[clap(short, long, default_value_t = 0.01)]
    step_size: f64,
    #[clap(short, long)]
    constant_time: bool,
    #[clap(short, long, default_value_t = 1.0)]
    duration: f64,
    #[clap(short, long, default_value_t = 0.0)]
    rotation: f64,
}

fn syntax_error(e: ParseError<LineCol>, filename: &str, source: &str) {
    let mut files = SimpleFiles::new();

    let file_id = files.add(filename, source);

    let diagnostic = Diagnostic::error().with_labels(vec![Label::primary(
        file_id,
        e.location.offset..e.location.offset + 1, // todo: better spans if possible
    )
    .with_message(format!("Expected {}", e.expected))]);

    let writer = StandardStream::stderr(ColorChoice::Always);
    let config = codespan_reporting::term::Config::default();

    term::emit(&mut writer.lock(), &config, &files, &diagnostic).unwrap();
}

fn main() {
    let args = Args::parse();

    let file = std::fs::read_to_string(&args.file).unwrap();

    let genesis_gear: Box<dyn Gear> = match gearbox_parser::gear_w(&file) {
        Ok(r) => r,
        Err(e) => {
            syntax_error(e, &args.file, &file);
            std::process::exit(1);
        }
    };

    let mut rot = args.rotation;
    loop {
        let start = Instant::now();

        if genesis_gear.turn(rot, 1) {
            rot += args.step_size;

            stdout().flush().unwrap();

            if args.constant_time {
                let sleep_time = args.duration - start.elapsed().as_secs_f64();
                if sleep_time > 0.0 {
                    sleep(Duration::from_secs_f64(sleep_time));
                }
            }

            print!("\r");
        } else {
            break;
        }
    }
}
