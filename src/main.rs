use std::path::{Component, PathBuf};

use walkdir::WalkDir;

const DESKTOP_DIR: &str = "C:/Users/yunzh/Desktop";
const INVALID_NAMES: [&str; 2] = ["node_modules", "env"];

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

fn main() {
    let input = std::env::args().nth(1).expect("No input").to_lowercase();

    let in_desktop_iter = WalkDir::new(DESKTOP_DIR).into_iter();

    let all_subfolders = in_desktop_iter
        .filter_map(|e| e.ok())
        .filter(|f| f.file_type().is_dir());

    let paths = all_subfolders.map(|f| f.into_path());

    let stripped_paths =
        paths.filter_map(|p| p.strip_prefix(DESKTOP_DIR).ok().map(|f| f.to_path_buf()));

    let viable_options = stripped_paths.filter_map(name_is_valid);

    let option_strings = viable_options.filter_map(|p| p.to_str().map(|s| s.to_string()));

    let mut count = 0;
    for option in option_strings {
        let lowercase = option.to_lowercase();

        if lowercase.contains(&input) {
            println!("{:?}", option);
            count += 1;
        }
    }

    println!("{input}");

    println!("{count}")
}
