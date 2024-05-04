use std::collections::HashMap;
use std::env;
use std::fs;
use std::io;
use std::io::BufRead;
use std::path::Path;
use std::process::exit;

struct CliOptions {
    dir_path: String,
    pattern: String,
}

struct Search {
    files: Vec<String>,
    pattern: String,
}

impl Search {
    fn normal(&self) -> HashMap<String, Vec<Vec<String>>> {
        let mut hashmap: HashMap<String, Vec<Vec<String>>> = HashMap::new();
        let files = &self.files;
        let pattern = &self.pattern.as_str();
        println!("reading files...");
        for file in files {
            let file_handler = fs::File::open(Path::new(&file.as_str())).unwrap();
            let reader = io::BufReader::new(file_handler);

            let mut line_no = 1;
            let mut v: Vec<Vec<String>> = Vec::new();

            for line in reader.lines() {
                let mut should_cont = true;
                let line = line.unwrap_or_else(|_| {
                    should_cont = false;
                    String::new()
                });
                if should_cont {
                    if line
                        .to_lowercase()
                        .contains(pattern.to_lowercase().as_str())
                    {
                        let line_no = format!("{}", line_no);
                        let data = line;
                        v.push(vec![line_no, data]);
                    }
                }
                line_no += 1;
            }
            hashmap.insert(String::from(file), v);
        }

        hashmap
    }

    fn display(hashmap: HashMap<String, Vec<Vec<String>>>) {
        for (k, v) in hashmap {
            for i in v {
                let p = format!("{}\n{}: {}", k, i[0], i[1]);
                println!("{p}\n");
            }
        }
    }
}

fn main() {
    let options = parse_cli_args();
    println!("INFO: Directory Path: {}", options.dir_path);
    let mut files = Vec::new();
    get_dir_items(
        &Path::new(format!("{}", options.dir_path).as_str()),
        &mut files,
    )
    .unwrap();

    let searcher = Search {
        files,
        pattern: options.pattern,
    };
    Search::display(Search::normal(&searcher));
}

fn parse_cli_args() -> CliOptions {
    let mut options = CliOptions {
        dir_path: String::new(),
        pattern: String::new(),
    };
    let args: Vec<String> = env::args().collect();
    for i in 0..args.len() {
        let t = &args[i];
        match t.as_str() {
            "-d" => {
                let path = String::from(&args[i + 1]);
                if Path::new(path.as_str()).is_dir() {
                    options.dir_path = path;
                } else {
                    println!("ERROR: {}: Invaild directory.", path);
                    exit(1)
                }
            }
            "-p" => {
                let p = String::from(&args[i + 1]);
                options.pattern = p;
            }
            _ => continue,
        }
    }
    options
}

fn get_dir_items(dir: &Path, files: &mut Vec<String>) -> Result<(), std::io::Error> {
    if dir.is_dir() {
        for item in fs::read_dir(dir)? {
            let item = item?;
            let path = item.path();
            if path.is_dir() {
                get_dir_items(&path, files)?
            } else {
                files.push(path.to_string_lossy().to_string())
            }
        }
    }
    Ok(())
}
