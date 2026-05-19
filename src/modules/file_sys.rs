use std::fs::{self, File};
use std::path::Path;
use walkdir::WalkDir;

/// Deeply walks directories starting from `path` to find a match.
pub fn find_file(name: &str, path: &Path) {
    println!("🔍 Searching for '{}' in '{}'...", name, path.display());
    let mut found = false;

    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        if entry.file_name().to_string_lossy() == name {
            println!("✅ Found: {}", entry.path().display());
            found = true;
        }
    }

    if !found {
        println!("❌ No files found matching '{}'.", name);
    }
}

/// Creates either an empty file or a nested folder tree structure.
pub fn create_item(path: &Path, is_dir: bool) {
    if is_dir {
        match fs::create_dir_all(path) {
            Ok(_) => println!("📂 Successfully created directory tree: {}", path.display()),
            Err(e) => eprintln!("❌ Failed to create directory: {}", e),
        }
    } else {
        // Ensure parent directories exist before making the file
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                let _ = fs::create_dir_all(parent);
            }
        }
        match File::create(path) {
            Ok(_) => println!("📄 Successfully created empty file: {}", path.display()),
            Err(e) => eprintln!("❌ Failed to create file: {}", e),
        }
    }
}

/// Moves or renames a file/folder flawlessly across platforms.
pub fn move_item(source: &Path, destination: &Path) {
    match fs::rename(source, destination) {
        Ok(_) => println!("🚚 Moved: {} -> {}", source.display(), destination.display()),
        Err(e) => eprintln!("❌ Error moving item: {}", e),
    }
}

/// Sends a file or directory safely to the system Recycle Bin / Trash.
pub fn delete_item(path: &Path) {
    match trash::delete(path) {
        Ok(_) => println!("🗑️ Safely moved '{}' to the system Trash Bin.", path.display()),
        Err(e) => eprintln!("❌ Failed to safely delete item: {}", e),
    }
}