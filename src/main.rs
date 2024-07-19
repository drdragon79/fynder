use std::{
    collections::HashMap,
    fs,
    io::{self, BufRead},
    path::Path
};
use clap::Parser;

#[derive(Parser)]
#[command(version)]
struct CliOptions {
    /// Directory path
    #[arg(long, short)]
    dir_path: String,

    /// Pattern
    #[arg(long, short)]
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
    let options = CliOptions::parse();
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

fn get_dir_items(dir: &Path, files: &mut Vec<String>) -> std::io::Result<()> {
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
