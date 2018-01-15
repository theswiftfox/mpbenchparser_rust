use std::io;
use std::env;
use std::fs::File;
use std::fs::{self};
use std::path::Path;
use std::path::PathBuf;
use std::io::prelude::*;

mod benchmark;

fn main() {
    let mut args = env::args();
    args.next(); // skip default argument
    let mut folders : Vec<String> = Vec::new();
    let working_dir = env::current_dir().unwrap();
    let delimiter = ",";

    while let Some(arg) = args.next() {
        folders.push(arg);
    }

    if folders.len() == 0 {
        let mut rel_path = String::new();
        println!("Current working dir: {}\r\nEnter relative path to the folder you want to process (use unix style seperators)!\r\nIf you want to process multiple folders, seperate the paths with a \";\"", working_dir.display());
        io::stdin().read_line(&mut rel_path).expect("Unable to read input");
        for path in rel_path.split(";") {
            folders.push(path.to_string());
        }
    }

    for folder in folders {
        let folder_path = combine_path(&working_dir, folder);
        println!("Processing {} ...", folder_path.display());
        let files = get_files(&folder_path).expect("error reading folder content");

        let mut combined : Vec<benchmark::Benchmark> = Vec::new();
        for file in files {
            let content = read_file(&file);
            let cur = benchmark::create_benchmark_from_data(&content).expect("unable to create benchmark from given data");
            //println!("{}\r\n+++++++++++++++++", cur);
            let mut config_found : bool = false;
            for i in 0 .. combined.len() {
                if combined[i].config == cur.config {
                    config_found = true;
                    let comb = benchmark::combine_benchmarks(&combined[i], &cur);
                    combined[i] = comb;
                }
            }
            if !config_found {
                combined.push(cur);
            }
            
        }

        let mut first = true;
        let filename = format!("{}/combined.csv", folder_path.display());
        let file = File::create(&filename).unwrap();;
        for fin in combined {
            if first {
                write!(&file, "{}\r\n{}", benchmark::header_string(&delimiter), benchmark::formatted_sections_string(&fin, &delimiter)).expect("unable to write content to file");
                first = false;
            } else {
                write!(&file, "{}", benchmark::formatted_sections_string(&fin, &delimiter)).expect("unable to write content");
            }
        }
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
            // println!("Found file: {}", entry.path().display());
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