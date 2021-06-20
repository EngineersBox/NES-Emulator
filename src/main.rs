mod cpu;
mod macros;

use std::fs;
use std::fs::{File, OpenOptions};
use std::sync::Mutex;

use lazy_static::lazy_static;
use slog::{Drain, Duplicate, Fuse, Logger};
use slog_async::{Async, OverflowStrategy};
use slog_json::Json;
use slog_term::{FullFormat, TermDecorator};

#[macro_use]
extern crate slog;
extern crate lazy_static;
extern crate slog_async;
extern crate slog_json;
extern crate slog_term;

fn initialize_logging() -> slog::Logger {
    let log_path: &str = "logs/";
    let directory_creation_message: &str;
    match fs::create_dir(log_path) {
        Ok(_) => {
            directory_creation_message = "Created logging directory";
        }
        Err(_) => {
            directory_creation_message = "Logging directory already exists, skipping";
        }
    }

    let log_file_path: String = format!("{}{}{}", log_path, chrono::Utc::now().to_string(), ".log");
    let file: File = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(log_file_path.as_str())
        .unwrap();

    let decorator: TermDecorator = TermDecorator::new().force_color().build();

    type FuseFFTD = Fuse<FullFormat<TermDecorator>>;
    type FuseJF = Fuse<Json<File>>;
    type FuseMD = Fuse<Mutex<Duplicate<FuseFFTD, FuseJF>>>;

    let d1: FuseFFTD = FullFormat::new(decorator).build().fuse();
    let d2: FuseJF = Json::default(file).fuse();
    let both: FuseMD = Mutex::new(Duplicate::new(d1, d2)).fuse();
    let both: Fuse<Async> = Async::new(both)
        .overflow_strategy(OverflowStrategy::Block)
        .build()
        .fuse();
    let log: Logger = Logger::root(both, o!());

    info!(log, "{}", directory_creation_message);
    log
}

lazy_static! {
    static ref LOGGER: Logger = initialize_logging();
}

fn main() {
    println!("Hello, world!");
}
