mod algo;

use std::{
    path::{Component, PathBuf},
    time::Instant,
};

use walkdir::WalkDir;

const DESKTOP_DIR: &str = "C:/Users/yunzh/Desktop";
const INVALID_NAMES: [&str; 2] = ["node_modules", "env"];

#[derive(Debug)]
struct SearchResult {
    path: PathBuf,
    score: i32,
}

fn name_is_valid(path: PathBuf) -> Option<PathBuf> {
    for component in path.components() {
        if let Component::Normal(s) = component {
            if let Some(str) = s.to_str() {
                if str.starts_with('.') || str.starts_with('_') || INVALID_NAMES.contains(&str) {
                    return None;
                }
            }
        }
    }
    Some(path)
}

fn get_folders(input: &str) {
    let pattern: Vec<char> = input.chars().collect();

    let in_desktop_iter = WalkDir::new(DESKTOP_DIR).into_iter();

    let all_subfolders = in_desktop_iter
        .filter_map(|e| e.ok())
        .filter(|f| f.file_type().is_dir());

    let paths = all_subfolders.map(|f| f.into_path());

    let stripped_paths =
        paths.filter_map(|p| p.strip_prefix(DESKTOP_DIR).ok().map(|f| f.to_path_buf()));

    let viable_paths = stripped_paths.filter_map(name_is_valid);

    let options: Vec<PathBuf> = viable_paths.collect();

    let now = Instant::now();

    let mut search_results: Vec<SearchResult> = vec![];
    for option in options {
        let string = option.to_str().expect("SHOULD BE OK");
        let res = algo::fuzzy_match(string, &pattern);

        search_results.push(SearchResult {
            path: option,
            score: res.score,
        })
    }

    search_results.sort_by(|a, b| a.score.cmp(&b.score));
    search_results.reverse();

    let items = 10;

    for result in &search_results[0..items] {
        println!("{:?}", result)
    }
    let elapsed = now.elapsed();
    println!("{:?}", elapsed);
}

fn main() {
    let input = std::env::args().nth(1).expect("No input").to_lowercase();

    get_folders(&input);
}
