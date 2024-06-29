use std::path::PathBuf;

use walkdir::WalkDir;

const DESKTOP_DIR: &str = "C:/Users/yunzh/Desktop";

fn main() {
    let in_desktop_iter = WalkDir::new(DESKTOP_DIR).into_iter();

    let all_subfolders = in_desktop_iter
        .filter_map(|e| e.ok())
        .filter(|f| f.file_type().is_dir());

    let paths = all_subfolders.map(|f| f.into_path());

    let stripped_paths =
        paths.filter_map(|p| p.strip_prefix(DESKTOP_DIR).ok().map(|f| f.to_path_buf()));

    let all_options: Vec<PathBuf> = stripped_paths.collect();

    for path in all_options.iter() {
        println!("{:?}", path.display());
    }
    println!("{:?}", all_options.len())
}
