use std::io;
use std::env;
use std::fs::File;
use std::fs::{self};
use std::path::Path;
use std::path::PathBuf;
use std::io::prelude::*;

mod benchmark;

fn main() {
    let mut rel_path = String::new();
    let mut full_path = env::current_dir().expect("unable to get current working dir");

    println!("Current working dir: {}\r\nEnter relative path to the folder you want to process (use unix style seperators):", full_path.display());
    io::stdin().read_line(&mut rel_path).expect("Unable to read input");
    full_path = combine_path(&full_path, rel_path);
    println!("{}", full_path.display());
    let files = get_files(&full_path).expect("error reading folder content");

    for file in files {
        let content = read_file(&file);
        let benchmark = benchmark::create_benchmark_from_data(content).expect("unable to create benchmark from given data");
        println!("Threads: {}\r\nReps: {}\r\nTime: {}\r\nDelay: {}", benchmark.config.threads, benchmark.config.outer_reps, benchmark.config.test_time, benchmark.config.delay);
    }
}

fn get_files(foldername: &Path) -> Result<Vec<PathBuf>, io::Error> {
    let mut result = Vec::new();
    if try!(fs::metadata(foldername)).is_dir() {
        for entry in try!(fs::read_dir(foldername)) {
            let entry = try!(entry);
            if try!(fs::metadata(entry.path())).is_dir() {
                continue;
            }
            println!("Found file: {}", entry.path().display());
            result.push(entry.path());
        }
        return Ok(result);
    }
    Err(io::Error::from(io::ErrorKind::InvalidInput))
}

fn read_file(filename: &Path) -> String {
    let mut file = File::open(filename).expect("File not found!");

    let mut content = String::new();
    file.read_to_string(&mut content).expect("unable to read file content");

    return content;
}

fn combine_path(workingdir: &Path, relpath: String) -> PathBuf {
    let mut out = workingdir.to_path_buf();
    let parts = relpath.split("/");
    for slice in parts {
        let slice = slice.trim();
        if slice == "." {
            continue;
        } else if slice == ".." {
            out = out.parent().unwrap().to_path_buf();
        } else {
            out.push(slice);
        }
    }
    return out;
}