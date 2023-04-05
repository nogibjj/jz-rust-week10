use std::env;
use std::path::PathBuf;
use walkdir::{DirEntry, WalkDir};

fn main() {
    // 获取命令行参数
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("Usage: file_search <directory> <filename/filetype>");
        return;
    }

    let directory = PathBuf::from(&args[1]);
    let search_term = &args[2];

    // 搜索目录中的文件
    for entry in WalkDir::new(&directory).into_iter().filter_entry(|e| !is_hidden(e)) {
        let entry = entry.unwrap();

        // 如果找到匹配的文件，打印其路径
        if matches(&entry, search_term).is_some() {
            println!("{}", entry.path().display());
        }
    }
}


// filter out hidden files
fn is_hidden(entry: &DirEntry) -> bool {
    entry.file_name()
        .to_str()
        .map(|s| s.starts_with('.'))
        .unwrap_or(false)
}

// check if file matches search term
fn matches(entry: &DirEntry, search_term: &str) -> Option<PathBuf> {
    let name = entry.file_name().to_string_lossy();

    if search_term.contains('.') {
        // 搜索文件类型
        let mut ext = String::new();
        entry.path().extension().map(|e| ext.push_str(&e.to_string_lossy()));
        if ext == search_term.trim_start_matches('.') {
            Some(entry.path().to_path_buf())
        } else {
            None
        }
    } else {
        // 搜索文件名
        if name.contains(search_term) {
            Some(entry.path().to_path_buf())
        } else {
            None
        }
    }
}
